use crate::card::CardContainer;

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