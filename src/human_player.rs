use std::collections::HashMap;
use sdl2::keyboard::Keycode;

use player::*;
use rstris::playfield::*;
use rstris::position::*;
use rstris::figure_pos::*;

static DELAY_FIRST_STEP_DOWN: u64 = 1 * 1000 * 1000 * 1000;

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
    last_forced_down_time: u64,
    delay_first_step_down: u64,
}

impl Player for HumanPlayer {

    fn common(&self) -> &PlayerCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut PlayerCommon {
        &mut self.common
    }

    fn figure_move_event(&mut self, ticks: u64, pf: &Playfield,
                         fig_pos: &FigurePos, movement: &Movement) {
    }


    fn new_figure_event(&mut self, ticks: u64,
                        pf: &Playfield, fig_pos: &FigurePos) {
        self.delay_first_step_down = ticks + DELAY_FIRST_STEP_DOWN;
    }


    fn handle_input(&mut self,
                    ticks: u64,
                    pressed_keys: &mut HashMap<Keycode, u64>) {
        let keys = pressed_keys.clone();
        for (key, pressed_at) in keys {
            match self.key_to_movement(key) {
                Some(movement) => {
                    let time_last_move =
                        self.common.time_since_move(ticks, &movement);
                    let time_pressed = ticks - pressed_at;
                    if ticks <= pressed_at {
                        self.common.add_move(movement, ticks);
                    } else if time_pressed > 200000000 &&
                        time_last_move > 50000000
                    {
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
            common: common,
            key_map: key_map,
            last_forced_down_time: 0,
            delay_first_step_down: 0,
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
