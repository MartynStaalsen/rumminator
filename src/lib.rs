pub mod engine;
pub mod players;

use std::fmt;
use anyhow::{Result, Error, anyhow};

// Core card system with unique IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardId(u16); // 3-digit ID: 011 = Ace of Spades (deck 1), 012 = Ace of Spades (deck 2)

impl CardId {
    pub fn new(base_card: u8, deck: u8) -> Self {
        // Format: DDC where DD=deck (01,02), C=card (1-52)
        let id = (deck as u16 * 100) + (base_card as u16);
        CardId(id)
    }
    
    pub fn deck(&self) -> u8 {
        (self.0 / 100) as u8
    }
    
    pub fn base_card(&self) -> u8 {
        (self.0 % 100) as u8
    }
}

// View of a card - for rendering and player interaction
#[derive(Debug, Clone, PartialEq)]
pub struct CardView {
    pub id: CardId,
    pub rank: Rank,
    pub suit: Suit,
    pub score_value: u32,
}

// Card rendering functions - static lookup from ID to card properties
pub fn card_id_to_rank(id: CardId) -> Rank {
    let base = id.base_card();
    match base {
        1..=13 => match base {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        },
        14..=26 => card_id_to_rank(CardId::new(base - 13, 1)), // Diamonds
        27..=39 => card_id_to_rank(CardId::new(base - 26, 1)), // Clubs
        40..=52 => card_id_to_rank(CardId::new(base - 39, 1)), // Spades
        53..=54 => Rank::Joker, // Two jokers
        _ => panic!("Invalid card base: {}", base),
    }
}

pub fn card_id_to_suit(id: CardId) -> Suit {
    let base = id.base_card();
    match base {
        1..=13 => Suit::Hearts,
        14..=26 => Suit::Diamonds,
        27..=39 => Suit::Clubs,
        40..=52 => Suit::Spades,
        53..=54 => Suit::None, // Jokers
        _ => panic!("Invalid card base: {}", base),
    }
}

pub fn card_id_to_score(id: CardId) -> u32 {
    let rank = card_id_to_rank(id);
    match rank {
        Rank::Ace | Rank::Two => 20,
        Rank::Joker => 50,
        Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        _ => rank as u32,
    }
}

pub fn card_id_to_view(id: CardId) -> CardView {
    CardView {
        id,
        rank: card_id_to_rank(id),
        suit: card_id_to_suit(id),
        score_value: card_id_to_score(id),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace = 1,
    Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7,
    Eight = 8, Nine = 9, Ten = 10, Jack = 11, Queen = 12, King = 13,
    Joker = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts, Diamonds, Clubs, Spades, None, // None for jokers
}

#[derive(Debug, Clone, PartialEq)]
pub struct CardContainer {
    pub cards: Vec<CardView>,
}

impl CardContainer {
    /// Returns true if this container forms a valid group (3+ same rank, different suits)
    pub fn is_valid_group(&self) -> bool {
        if self.cards.len() < 3 {
            return false;
        }
        
        // TODO: Implement group validation logic
        // All cards must have same rank (accounting for wilds)
        // Must have at least 3 cards
        // Can't have duplicate suit+rank combinations (except wilds)
        true // Placeholder
    }
    
    /// Returns true if this container forms a valid run (4+ sequential same suit)
    pub fn is_valid_run(&self) -> bool {
        if self.cards.len() < 4 {
            return false;
        }
        
        // TODO: Implement run validation logic
        // All cards must be same suit (accounting for wilds)
        // Must form sequential ranks (accounting for wilds)
        // Must have at least 4 cards
        true // Placeholder
    }
}

// Container registry system - ground truth for card locations
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerId(String);

impl ContainerId {
    pub fn deck() -> Self { ContainerId("deck".to_string()) }
    pub fn discard() -> Self { ContainerId("discard".to_string()) }
    pub fn table_temp() -> Self { ContainerId("table_temp".to_string()) }
    
    pub fn player_hand(player: usize) -> Self {
        ContainerId(format!("player_{}_hand", player))
    }
    
    pub fn player_group(player: usize, group_idx: usize) -> Self {
        ContainerId(format!("player_{}_group_{}", player, group_idx))
    }
    
    pub fn player_run(player: usize, run_idx: usize) -> Self {
        ContainerId(format!("player_{}_run_{}", player, run_idx))
    }
}

#[derive(Debug)]
pub struct CardRegistry {
    card_locations: HashMap<CardId, ContainerId>,
}

impl CardRegistry {
    pub fn new() -> Self {
        Self {
            card_locations: HashMap::new(),
        }
    }
    
    /// Initialize registry with all cards in deck
    pub fn initialize_with_deck(&mut self) -> Vec<CardId> {
        let mut all_cards = Vec::new();
        
        // Create 2 decks (52 cards + 2 jokers each = 108 total)
        for deck in 1..=2 {
            for base_card in 1..=54 {
                let card_id = CardId::new(base_card, deck);
                self.card_locations.insert(card_id, ContainerId::deck());
                all_cards.push(card_id);
            }
        }
        
        all_cards
    }
    
    pub fn move_card(&mut self, card_id: CardId, to_container: ContainerId) -> Result<()> {
        if self.card_locations.contains_key(&card_id) {
            self.card_locations.insert(card_id, to_container);
            Ok(())
        } else {
            Err(anyhow!("Card {:?} not found in registry", card_id))
        }
    }
    
    pub fn get_location(&self, card_id: CardId) -> Option<&ContainerId> {
        self.card_locations.get(&card_id)
    }
    
    pub fn get_cards_in_container(&self, container: &ContainerId) -> Vec<CardId> {
        self.card_locations
            .iter()
            .filter(|(_, loc)| *loc == container)
            .map(|(card, _)| *card)
            .collect()
    }
    
    pub fn get_container_view(&self, container: &ContainerId) -> CardContainer {
        let card_ids = self.get_cards_in_container(container);
        let cards = card_ids.into_iter().map(card_id_to_view).collect();
        CardContainer { cards }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetType {
    Group,  // Same rank, different suits (min 3 cards)
    Run,    // Sequential ranks, same suit (min 4 cards)
}

impl SetType {
    pub fn min_size(&self) -> usize {
        match self {
            SetType::Group => 3,
            SetType::Run => 4,
        }
    }

    pub fn validate(&self, container: &CardContainer) -> bool {
        match self {
            SetType::Group => container.is_valid_group(),
            SetType::Run => container.is_valid_run(),
        }
    }
}

/// Contract types for each hand
#[derive(Debug, Clone, PartialEq)]
pub enum ContractOrder {
    GG,           // Hand 1: GG
    GR,         // Hand 2: GR  
    RR,             // Hand 3: RR
    GGG,         // Hand 4: GGG
    GGR,     // Hand 5: GGR
    GRR,     // Hand 6: GRR
    RRR,           // Hand 7: RRR
}

impl ContractOrder {
    pub fn from_hand_number(hand_number: usize) -> Self {
        match hand_number {
            1 => ContractOrder::GG,
            2 => ContractOrder::GR,
            3 => ContractOrder::RR,
            4 => ContractOrder::GGG,
            5 => ContractOrder::GGR,
            6 => ContractOrder::GRR,
            7 => ContractOrder::RRR,
            _ => panic!("Invalid hand number: {}", hand_number),
        }
    }
    
    pub fn hand_size(&self) -> usize {
        match self {
            ContractOrder::GG |
            ContractOrder::GR |
            ContractOrder::RR |
            ContractOrder::GGG => 10,
            
            ContractOrder::GGR |
            ContractOrder::GRR |
            ContractOrder::RRR => 12,
        }
    }
    
    pub fn required_groups(&self) -> usize {
        match self {
            ContractOrder::GG => 2,
            ContractOrder::GR => 1,
            ContractOrder::RR => 0,
            ContractOrder::GGG => 3,
            ContractOrder::GGR => 2,
            ContractOrder::GRR => 1,
            ContractOrder::RRR => 0,
        }
    }
    
    pub fn required_runs(&self) -> usize {
        match self {
            ContractOrder::GG => 0,
            ContractOrder::GR => 1,
            ContractOrder::RR => 2,
            ContractOrder::GGG => 0,
            ContractOrder::GGR => 1,
            ContractOrder::GRR => 2,
            ContractOrder::RRR => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContractBid {
    pub groups: Vec<CardContainer>,
    pub runs: Vec<CardContainer>,
}

impl ContractBid {
    pub fn new(groups: Vec<CardContainer>, runs: Vec<CardContainer>) -> Self {
        Self { groups, runs }
    }
    
    /// Validates if this bid satisfies the given contract order
    pub fn validate(&self, contract: &ContractOrder) -> bool {
        // Check correct number of groups and runs
        if self.groups.len() != contract.required_groups() ||
           self.runs.len() != contract.required_runs() {
            return false;
        }
        
        // Validate each group is actually a valid group
        for group in &self.groups {
            if !group.is_valid_group() {
                return false;
            }
        }
        
        // Validate each run is actually a valid run
        for run in &self.runs {
            if !run.is_valid_run() {
                return false;
            }
        }
        
        true
    }
}


// Player state is now tracked via registry containers
// Table state is tracked via registry containers

#[derive(Debug, Clone, PartialEq)]
pub enum GamePhase {
    PreTurn,      // Before draw, checking for nunu
    Drawing,      // Player choosing draw source
    Playing,      // Optional laying down / table manipulation
    Discarding,   // Must discard to end turn
    GameOver,
}

#[derive(Debug)]
pub struct GameState {
    pub card_registry: CardRegistry, // All card locations - the ground truth
    pub current_player: usize,
    pub contract: ContractOrder,
}

// Simplified move system
#[derive(Debug, Clone)]
pub enum DrawDecision {
    Deck,
    Discard,
}

#[derive(Debug, Clone)]
pub struct CardMove {
    pub card_id: CardId,
    pub to_container: ContainerId,
}

#[derive(Debug, Clone)]
pub struct TurnResult {
    pub move_ledger: Vec<CardMove>,
    pub discard: CardId,
}


// Player trait for new control flow
pub trait Player: fmt::Debug {
    /// Called when another player discards - return true if you want to nunu
    fn check_nunu(&mut self, view: &PlayerView, discarded_card: &CardView) -> bool;
    
    /// Called for active player to decide draw source
    fn draw_decision(&mut self, view: &PlayerView) -> DrawDecision;
    
    /// Called for active player's main turn - return move ledger and discard
    fn play_turn(&mut self, view: &PlayerView) -> Result<TurnResult>;
    
    /// Called to notify about game state changes
    fn notify_game_update(&mut self, view: &PlayerView);
}

// TODO: Simplified for now - will be expanded later
#[derive(Debug, Clone)]
pub struct PlayerView {
    pub current_player: usize,  // who's turn it is. 0 means mine, 1 is player to my left etc
    pub contract: ContractOrder,
    pub held_cards: CardContainer,           // Player's own cards
    pub table_groups: Vec<CardContainer>, // All laid down groups  
    pub table_runs: Vec<CardContainer>,   // All laid down runs
    pub last_discard: CardView, // Top of discard pile (none if "dead")
    // TODO: info about other players (nunu requests, held hand size, laid down)
    // TODO: discard ledger
    // TODO: scores
}


