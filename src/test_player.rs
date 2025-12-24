use anyhow::{Result, anyhow};
use crate::{Player, PlayerView, DrawDecision, TurnResult, CardView};

// Simple player that always draws from deck and discards highest card
#[derive(Debug)]
pub struct BasicPlayer;

impl BasicPlayer {
    pub fn new() -> Self {
        Self
    }
}

impl Player for BasicPlayer {
    fn check_nunu(&mut self, _view: &PlayerView, _discarded_card: &CardView) -> bool {
        // Never nunu for basic player
        false
    }
    
    fn draw_decision(&mut self, _view: &PlayerView) -> DrawDecision {
        // Always draw from deck for simplicity
        DrawDecision::Deck
    }
    
    fn play_turn(&mut self, view: &PlayerView) -> Result<TurnResult> {
        // For now, don't make any table moves, just discard highest card
        let highest_card = self.find_highest_card(&view.held_cards)?;
        
        Ok(TurnResult {
            move_ledger: Vec::new(), // No table manipulation
            discard: highest_card.id,
        })
    }
    
    fn notify_game_update(&mut self, _view: &PlayerView) {
        // Basic player doesn't need to track state
    }
}

impl BasicPlayer {
    fn find_highest_card<'a>(&self, hand: &'a crate::card::CardContainer) -> Result<&'a CardView> {
        if hand.cards.is_empty() {
            return Err(anyhow!("Hand is empty"));
        }
        
        // Find card with highest score value
        hand.cards.iter()
            .max_by_key(|card| card.score_value)
            .ok_or_else(|| anyhow!("No cards in hand"))
    }
}