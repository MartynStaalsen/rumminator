pub mod card;
pub mod contract; 
pub mod player;
pub mod engine;
pub mod test_player;

// Re-export commonly used types
pub use card::{CardId, CardView, CardContainer, CardRegistry, ContainerId, SetType};
pub use contract::{ContractOrder, ContractBid};
pub use player::{Player, PlayerView, DrawDecision, CardMove, TurnResult};

// Game state types
#[derive(Debug)]
pub struct GameState {
    pub card_registry: CardRegistry, // All card locations - the ground truth
    pub current_player: usize,
    pub contract: ContractOrder,
}