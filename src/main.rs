mod engine;
mod players;

use anyhow::Result;
use engine::hand;
use players::BasicPlayer;

fn main() -> Result<()> {
    println!("🃏 Starting Rumminator Game Engine");
    
    // Create basic players (testing with fewer for now)
    let players: Vec<Box<dyn rumminator::Player>> = vec![
        Box::new(BasicPlayer::new()),
        Box::new(BasicPlayer::new()),
    ];
    
    println!("Players: 2 BasicPlayers");
    
    // Run a single hand
    println!("🎮 Starting hand 1...");
    match hand(players, 1) {
        Ok(()) => {
            println!("🏁 Hand test completed!");
        }
        Err(e) => {
            println!("❌ Hand error: {}", e);
        }
    }
    
    Ok(())
}