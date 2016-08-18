use std::collections::HashMap;
use sdl2::keyboard::Keycode;

use player::*;
use rstris::position::*;

pub struct KeyMap {
    pub step_left: Option<Keycode>,
    pub step_right: Option<Keycode>,
    pub step_down: Option<Keycode>,
    pub rot_cw: Option<Keycode>,
    pub rot_ccw: Option<Keycode>,
}

pub struct HumanPlayer {
    common: PlayerCommon,
    key_map: KeyMap,
}

impl Player for HumanPlayer {

    fn common(&self) -> &PlayerCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut PlayerCommon {
        &mut self.common
    }

    fn handle_input(&mut self, current_ticks: u64,
                    pressed_keys: &mut HashMap<Keycode, u64>)
                    -> Vec<(Movement, u64)> {
        let mut moves: Vec<(Movement, u64)> = vec![];
        let keys = pressed_keys.clone();
        for (key, pressed_at) in keys {
            match self.key_to_movement(key) {
                Some(movement) => {
                    let time_last_move = current_ticks -
                        match self.common.time_last_move.get(&movement) {
                            Some(t) => *t,
                            None => 0
                        };
                    let time_pressed = current_ticks - pressed_at;
                    if current_ticks <= pressed_at {
                        moves.push((movement, current_ticks));
                    } else if time_pressed > 200000000 &&
                        time_last_move > 50000000
                    {
                        moves.push((movement, current_ticks));
                    }
                }
                None => {}
            }
        }
        return moves;
    }
}

impl HumanPlayer {
    pub fn new(common: PlayerCommon, key_map: KeyMap) -> Self {
        HumanPlayer {
            common: common,
            key_map: key_map,
        }
    }
    fn key_to_movement(&self, key: Keycode) -> Option<Movement> {
        if !self.key_map.step_left.is_none() &&
            key == self.key_map.step_left.unwrap()
        {
            return Some(Movement::MoveLeft);
        } else if !self.key_map.step_right.is_none() &&
            key == self.key_map.step_right.unwrap()
        {
            return Some(Movement::MoveRight);
        } else if !self.key_map.step_down.is_none() &&
            key == self.key_map.step_down.unwrap()
        {
            return Some(Movement::MoveDown);
        } else if !self.key_map.rot_cw.is_none() &&
            key == self.key_map.rot_cw.unwrap()
        {
            return Some(Movement::RotateCW);
        }
        else if !self.key_map.rot_ccw.is_none() &&
            key == self.key_map.rot_ccw.unwrap()
        {
            return Some(Movement::RotateCCW);
        }
        return None;
    }
}
