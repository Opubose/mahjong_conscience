use crate::error::{EngineError, EngineResult};
use crate::event::GameEvent;
use crate::phase::GamePhase;
use crate::state::GameState;
use crate::types::TileID;

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
        let PlayerAction::Discard(discarded_tile) = action else {
            return Err(EngineError::Internal("Expected Discard action".to_string()));
        };

        let player = &mut self.state.players[player_id];

        let tile_idx = player.hand.iter().position(|&t| t == discarded_tile)
            .ok_or_else(|| EngineError::TileNotFound(player_id))?;

        player.hand.remove(tile_idx);
        player.discards.push(discarded_tile);

        self.phase = GamePhase::CallWait {
            discarded_by: player_id,
            discarded_tile,
        };

        Ok(vec![GameEvent::TileDiscarded {
            player_id,
            tile: discarded_tile,
            is_tsumogiri: false, // TODO: track if the discarded tile was drawn this turn or not
            is_riichi: player.is_riichi,
        }])
    }
    
    fn process_call(&mut self, player_id: usize, action: PlayerAction) -> EngineResult<Vec<GameEvent>> {
        // verify player can call
        // if call is valid, update hand and melds
        // if call is winning, update score and end round
        // if no calls, change phase back to DiscardWait for next player
        todo!()
    }
}
