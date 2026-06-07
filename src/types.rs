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
        let id = self.0;

        match id {
            0..=35 => Tile::Suited {
                suit: Suit::Man,
                rank: (id / 4) + 1,
            },
            36..=71 => Tile::Suited {
                suit: Suit::Pin,
                rank: ((id - 36) / 4) + 1,
            },
            72..=107 => Tile::Suited {
                suit: Suit::Sou,
                rank: ((id - 72) / 4) + 1,
            },
            108..=123 => {
                let wind_idx = (id - 108) / 4;
                let wind = match wind_idx {
                    0 => Wind::East,
                    1 => Wind::South,
                    2 => Wind::West,
                    _ => Wind::North,
                };
                Tile::Wind(wind)
            },
            124..=135 => {
                let dragon_idx = (id - 124) / 4;
                let dragon = match dragon_idx {
                    0 => Dragon::Red,
                    1 => Dragon::Green,
                    _ => Dragon::White,
                };
                Tile::Dragon(dragon)
            },
            _ => panic!("Invalid TileID"),
        }
    }

    pub fn is_red_five(&self) -> bool {
        // IDs 16, 52, 88 are red fives
        matches!(self.0, 16 | 52 | 88)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeldType {
    Chi,
    Pon,
    OpenKan,    // Kan from a discard
    ClosedKan,  // concealed Kan drawn from the wall
    AddedKan,   // upgrading an existing open Pon to a Kan
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meld {
    pub meld_type: MeldType,
    pub tiles: Vec<TileID>,  // the tiles that form the meld
    pub called_tile: Option<TileID>, // the tile that was called to form the meld (None for closed Kan)
    pub from_player: Option<usize>, // the player index from whom the tile was called (None for closed Kan)
}
