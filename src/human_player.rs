use sdl2::keyboard::Keycode;
use std::collections::HashMap;

use player::*;
use rstris::figure_pos::*;
use rstris::movement::*;
use rstris::playfield::*;

static DELAY_FIRST_STEP_DOWN: u64 = 1000_000_000;

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
    delay_first_step_down: u64,
}

impl Player for HumanPlayer {
    fn common(&self) -> &PlayerCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut PlayerCommon {
        &mut self.common
    }

    fn figure_move_event(&mut self, _: u64, _: &Playfield, _: &FigurePos, _: &Movement) {}

    fn new_figure_event(&mut self, ticks: u64, _: &Playfield, _: &FigurePos) {
        self.delay_first_step_down = ticks + DELAY_FIRST_STEP_DOWN;
    }

    fn handle_input(&mut self, ticks: u64, pressed_keys: &mut HashMap<Keycode, u64>) {
        let keys = pressed_keys.clone();
        for (key, pressed_at) in keys {
            match self.key_to_movement(key) {
                Some(movement) => {
                    let time_last_move = self.common.time_since_move(ticks, &movement);
                    let time_pressed = ticks - pressed_at;
                    if ticks <= pressed_at {
                        self.common.add_move(movement, ticks);
                    } else if time_pressed > 200000000 && time_last_move > 50000000 {
                        self.common.add_move(movement, ticks);
                    }
                }
                None => {}
            }
        }
    }
}

impl HumanPlayer {
    pub fn new(common: PlayerCommon, key_map: KeyMap) -> Self {
        HumanPlayer {
            common,
            key_map,
            delay_first_step_down: 0,
        }
    }
    fn key_to_movement(&self, key: Keycode) -> Option<Movement> {
        if self.key_map.step_left.is_some() && key == self.key_map.step_left.unwrap() {
            return Some(Movement::MoveLeft);
        } else if self.key_map.step_right.is_some() && key == self.key_map.step_right.unwrap() {
            return Some(Movement::MoveRight);
        } else if self.key_map.step_down.is_some() && key == self.key_map.step_down.unwrap() {
            return Some(Movement::MoveDown);
        } else if self.key_map.rot_cw.is_some() && key == self.key_map.rot_cw.unwrap() {
            return Some(Movement::RotateCW);
        } else if self.key_map.rot_ccw.is_some() && key == self.key_map.rot_ccw.unwrap() {
            return Some(Movement::RotateCCW);
        }
        return None;
    }
}
