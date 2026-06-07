use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileID(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Man,
    Pin,
    Sou,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dragon {
    Red,
    Green,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tile {
    Suited { suit: Suit, rank: u8 },    // rank 1-9
    Wind(Wind),
    Dragon(Dragon),
}

impl TileID {
    pub fn as_logical(&self) -> Tile {
        // convert ID 0-135 to logical tile
        // 0-35 - Man
        // 36-71 - Pin
        // 72-107 - Sou
        // 108-123 - Winds (4 types x 4 copies)
        // 124-135 - Dragons (3 types x 4 copies)
        todo!()
    }

    pub fn is_red_five(&self) -> bool {
        // IDs 16, 52, 88 are red fives
        todo!()
    }
}
