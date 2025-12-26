use std::collections::HashMap;
use anyhow::{Result, anyhow};

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

// Container system
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerId(pub String);

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