use crate::types::{TileID, Wind, Meld};

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub hand: Vec<TileID>,
    pub discards: Vec<TileID>,
    pub called_melds: Vec<Meld>,
    pub score: i32,
    pub wind: Wind,
    pub is_riichi: bool,
    pub is_furiten: bool,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub players: [PlayerState; 4],
    pub wall: Vec<TileID>,
    pub dead_wall: Vec<TileID>,
    pub dora_indicators: Vec<TileID>,
    pub round_wind: Wind,
    pub round_number: u8,
    pub honba: u8,
    pub riichi_sticks: u8,
    pub current_turn: usize,
}
