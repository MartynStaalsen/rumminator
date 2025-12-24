use std::fmt;
use anyhow::Result;
use crate::card::CardView;

/// Player trait for new control flow
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

// Simplified move system
#[derive(Debug, Clone)]
pub enum DrawDecision {
    Deck,
    Discard,
}

#[derive(Debug, Clone)]
pub struct CardMove {
    pub card_id: crate::card::CardId,
    pub to_container: crate::card::ContainerId,
}

#[derive(Debug, Clone)]
pub struct TurnResult {
    pub move_ledger: Vec<CardMove>,
    pub discard: crate::card::CardId,
}

// TODO: Simplified for now - will be expanded later
#[derive(Debug, Clone)]
pub struct PlayerView {
    pub current_player: usize,  // who's turn it is. 0 means mine, 1 is player to my left etc
    pub contract: crate::contract::ContractOrder,
    pub held_cards: crate::card::CardContainer,           // Player's own cards
    pub table_groups: Vec<crate::card::CardContainer>, // All laid down groups  
    pub table_runs: Vec<crate::card::CardContainer>,   // All laid down runs
    pub last_discard: CardView, // Top of discard pile (none if "dead")
    // TODO: info about other players (nunu requests, held hand size, laid down)
    // TODO: discard ledger
    // TODO: scores
}