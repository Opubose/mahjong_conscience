use crate::error::{EngineError, EngineResult};
use crate::event::GameEvent;
use crate::phase::GamePhase;
use crate::state::GameState;
use crate::types::{Tile, TileID};

pub enum PlayerAction {
    Discard(TileID),
    CallPon,
    CallChi,
    CallKan,
    DeclareRiichi(TileID),
    DeclareTsumo,
    DeclareRon,
    Pass,
}

pub struct Engine {
    pub state: GameState,
    pub phase: GamePhase,
}

impl Engine {
    pub fn handle_action(&mut self, player_id: usize, action: PlayerAction) -> EngineResult<Vec<GameEvent>> {
       match self.phase {
            GamePhase::DiscardWait { player_id: active_player } => {
                if player_id != active_player {
                    return Err(EngineError::NotYourTurn(player_id));
                }
                self.process_discard(player_id, action)
            }
            GamePhase::CallWait { .. } => {
                self.process_call(player_id, action)
            }
            _ => Err(EngineError::InvalidPhase),
       } 
    }

    fn process_discard(&mut self, player_id: usize, action: PlayerAction) -> EngineResult<Vec<GameEvent>> {
        // verify player has tile
        // remove tile from hand, add to discards
        // check if tile is a winning tile for anyone
        // change phase to CallWait
        // emit TileDiscarded event
        todo!()
    }
    
    fn process_call(&mut self, player_id: usize, action: PlayerAction) -> EngineResult<Vec<GameEvent>> {
        // verify player can call
        // if call is valid, update hand and melds
        // if call is winning, update score and end round
        // if no calls, change phase back to DiscardWait for next player
        todo!()
    }
}