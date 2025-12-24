use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::{
    GameState, ContractOrder, Player, PlayerView, DrawDecision, 
    CardMove, CardRegistry, ContainerId, CardId, CardView, CardContainer
};
use crate::card::{card_id_to_view, Rank, Suit};

#[derive(Debug)]
enum TurnOutcome {
    Continue,
    HandEnd,
}

pub fn hand(mut players: Vec<Box<dyn Player>>, hand_number: usize) -> Result<()> {
    let mut state = initialize_game(players.len(), hand_number)?;
    
    // TODO: For now just run a few turns to test the system
    for _turn in 0..5 {
        match run_turn(&mut players, &mut state)? {
            TurnOutcome::Continue => continue,
            TurnOutcome::HandEnd => {
                println!("Hand ended!");
                return Ok(());
            }
        }
    }
    
    println!("Completed 5 turns successfully");
    Ok(())
}

fn initialize_game(num_players: usize, hand_number: usize) -> Result<GameState> {
    let mut card_registry = CardRegistry::new();
    let all_cards = card_registry.initialize_with_deck();
    
    // Shuffle the card IDs
    let mut shuffled_cards = all_cards;
    shuffled_cards.shuffle(&mut thread_rng());
    
    let contract = ContractOrder::from_hand_number(hand_number);
    let hand_size = contract.hand_size();
    
    // Deal cards to player hands
    let mut card_idx = 0;
    for player_id in 0..num_players {
        for _ in 0..hand_size {
            if card_idx >= shuffled_cards.len() {
                return Err(anyhow!("Not enough cards to deal"));
            }
            card_registry.move_card(
                shuffled_cards[card_idx], 
                ContainerId::player_hand(player_id)
            )?;
            card_idx += 1;
        }
    }
    
    // First discard
    if card_idx >= shuffled_cards.len() {
        return Err(anyhow!("No cards left for initial discard"));
    }
    card_registry.move_card(shuffled_cards[card_idx], ContainerId::discard())?;
    card_idx += 1;
    
    // Rest go to deck
    for &card_id in &shuffled_cards[card_idx..] {
        card_registry.move_card(card_id, ContainerId::deck())?;
    }
    
    Ok(GameState {
        card_registry,
        current_player: 0,
        contract,
    })
}

fn run_turn(players: &mut [Box<dyn Player>], state: &mut GameState) -> Result<TurnOutcome> {
    let num_players = players.len();
    
    println!("Turn {}: Player {} playing", state.current_player, state.current_player);
    
    // 1. TODO: Poll for nunu - simplified for now
    
    // 2. Active player decides draw source
    let view = generate_view(state, state.current_player);
    let draw_decision = players[state.current_player].draw_decision(&view);
    execute_draw(state, draw_decision)?;
    
    // 3. Active player plays their turn
    let updated_view = generate_view(state, state.current_player);
    let turn_result = players[state.current_player].play_turn(&updated_view)?;
    
    // 4. Validate and execute move ledger
    validate_move_ledger(state, &turn_result.move_ledger)?;
    execute_move_ledger(state, turn_result.move_ledger)?;
    
    // 5. Execute discard
    execute_discard(state, turn_result.discard)?;
    
    // 6. Check for hand end
    if is_hand_over(state) {
        return Ok(TurnOutcome::HandEnd);
    }
    
    // 7. Advance to next player
    state.current_player = (state.current_player + 1) % num_players;
    
    Ok(TurnOutcome::Continue)
}

fn execute_draw(state: &mut GameState, decision: DrawDecision) -> Result<()> {
    let card_id = match decision {
        DrawDecision::Deck => {
            let deck_cards = state.card_registry.get_cards_in_container(&ContainerId::deck());
            *deck_cards.last().ok_or_else(|| anyhow!("Deck is empty"))?
        }
        DrawDecision::Discard => {
            let discard_cards = state.card_registry.get_cards_in_container(&ContainerId::discard());
            *discard_cards.last().ok_or_else(|| anyhow!("Discard pile is empty"))?
        }
    };
    
    // Move card to current player's hand
    state.card_registry.move_card(card_id, ContainerId::player_hand(state.current_player))?;
    
    // TODO: Handle nunu - simplified for now
    
    Ok(())
}

fn validate_move_ledger(state: &GameState, moves: &[CardMove]) -> Result<()> {
    // Basic validation - ensure player owns the cards they're moving
    for card_move in moves {
        if let Some(current_location) = state.card_registry.get_location(card_move.card_id) {
            // Check if card is in current player's containers
            let player_hand = ContainerId::player_hand(state.current_player);
            // TODO: Add more sophisticated validation
            if current_location != &player_hand {
                // Allow moves from player's own laid-down sets too
                // This is a simplified check for now
            }
        } else {
            return Err(anyhow!("Card {:?} not found", card_move.card_id));
        }
    }
    
    Ok(())
}

fn execute_move_ledger(state: &mut GameState, moves: Vec<CardMove>) -> Result<()> {
    for card_move in moves {
        state.card_registry.move_card(card_move.card_id, card_move.to_container)?;
    }
    Ok(())
}

fn execute_discard(state: &mut GameState, card_id: CardId) -> Result<()> {
    state.card_registry.move_card(card_id, ContainerId::discard())?;
    Ok(())
}

fn is_hand_over(state: &GameState) -> bool {
    // Check if current player has no cards left
    let hand_cards = state.card_registry.get_cards_in_container(&ContainerId::player_hand(state.current_player));
    hand_cards.is_empty()
}

/// Intelligently generates a PlayerView by interrogating game state
fn generate_view(state: &GameState, player_index: usize) -> PlayerView {
    // Step 1: Determine how many players are actually in the game
    let num_players = discover_player_count(state);
    
    // Step 2: Calculate relative current player position
    let relative_current_player = calculate_relative_position(
        state.current_player, 
        player_index, 
        num_players
    );
    
    // Step 3: Get this player's hand
    let held_cards = state.card_registry.get_container_view(
        &ContainerId::player_hand(player_index)
    );
    
    // Step 4: Discover all active table sets by scanning registry
    let (table_groups, table_runs) = discover_table_sets(state, num_players);
    
    // Step 5: Get the current discard (if any)
    let last_discard = get_current_discard(state);
    
    PlayerView {
        current_player: relative_current_player,
        contract: state.contract.clone(),
        held_cards,
        table_groups,
        table_runs,
        last_discard,
    }
}

/// Discover how many players are actually in this game by checking for non-empty hands
fn discover_player_count(state: &GameState) -> usize {
    let mut max_player = 0;
    
    // Check up to 4 possible players
    for player_id in 0..4 {
        let hand_cards = state.card_registry.get_cards_in_container(
            &ContainerId::player_hand(player_id)
        );
        if !hand_cards.is_empty() {
            max_player = player_id;
        }
    }
    
    max_player + 1 // Convert from 0-based to count
}

/// Calculate relative position of current player from this player's perspective
fn calculate_relative_position(current: usize, viewing_player: usize, num_players: usize) -> usize {
    if current >= viewing_player {
        current - viewing_player
    } else {
        // Wrap around: if current=0 and I'm player 2, they're 2 positions behind me
        // which means num_players - (viewing_player - current) positions ahead
        num_players - (viewing_player - current)
    }
}

/// Discover all active groups and runs on the table by scanning the registry
fn discover_table_sets(state: &GameState, num_players: usize) -> (Vec<CardContainer>, Vec<CardContainer>) {
    let mut table_groups = Vec::new();
    let mut table_runs = Vec::new();
    
    // Scan all possible containers for each player
    for player_id in 0..num_players {
        // Check for groups (up to reasonable limit)
        for group_idx in 0..5 { // Most contracts have max 3 groups, so 5 is safe
            let container_id = ContainerId::player_group(player_id, group_idx);
            let container_view = state.card_registry.get_container_view(&container_id);
            
            if !container_view.cards.is_empty() {
                table_groups.push(container_view);
            }
        }
        
        // Check for runs (up to reasonable limit)
        for run_idx in 0..5 { // Most contracts have max 3 runs, so 5 is safe
            let container_id = ContainerId::player_run(player_id, run_idx);
            let container_view = state.card_registry.get_container_view(&container_id);
            
            if !container_view.cards.is_empty() {
                table_runs.push(container_view);
            }
        }
    }
    
    (table_groups, table_runs)
}

/// Get the current top discard, handling edge cases gracefully
fn get_current_discard(state: &GameState) -> CardView {
    let discard_cards = state.card_registry.get_cards_in_container(&ContainerId::discard());
    
    if let Some(&top_discard_id) = discard_cards.last() {
        card_id_to_view(top_discard_id)
    } else {
        // No discards yet - this can happen at start of game
        // Return a dummy card that's clearly invalid
        CardView {
            id: CardId::new(255, 255), // Invalid ID
            rank: Rank::Ace,
            suit: Suit::None,
            score_value: 0,
        }
    }
}

