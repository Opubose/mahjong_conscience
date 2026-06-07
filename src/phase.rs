use serde::{Serialize, Deserialize};

// state machine for game phases
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    RoundSetup,                                                             // waiting to start round

    DiscardWait { player_id: usize },                                       // player has drawn a tile and is thinking about what to discard
    
    CallWait { discarded_by: usize, discarded_tile: crate::types::TileID }, // a tile was just discarded, waiting to see if anyone calls Pon, Chi, Kan, or Ron
    
    RoundEnd,                                                               // round has ended, waiting to tally scores and start next round
}
