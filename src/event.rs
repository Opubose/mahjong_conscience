use serde::{Serialize, Deserialize};
use crate::types::{TileID, Wind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    RoundStarted {
        round_wind: Wind,
        round_number: u8,
        dealer_id: usize,
        starting_hands: [Vec<TileID>; 4],
    },
    TileDrawn {
        player_id: usize,
        tile: TileID,
    },
    TileDiscarded {
        player_id: usize,
        tile: TileID,
        is_tsumogiri: bool,
        is_riichi: bool,
    },
    MeldCalled {
        player_id: usize,
    },
    PlayerWon {
        winner_id: usize,
        from_player_id: Option<usize>,  // none if tsumo, some if ron
        winning_tile: TileID,
        score_change: [i32; 4],
    },
    ExhaustiveDraw,
}