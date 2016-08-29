extern crate rand;

use rstris::playfield::*;
use rstris::position::*;
use rstris::figure_pos::*;
use rstris::find::*;

use player::*;

pub enum ComputerType {
    RandomStupid,
}

pub struct ComputerPlayer {
    common: PlayerCommon,
    moves: Vec<(Movement, u64)>,
    last_fig: String,
    avail_pos: Vec<Position>,
    last_path_update: u64,
    time_next_move: u64,
    com_type: ComputerType,
}

impl Player for ComputerPlayer {
    fn common(&self) -> &PlayerCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut PlayerCommon {
        &mut self.common
    }

    fn update(&mut self, current_ticks: u64, pf: &Playfield) {
        match self.common.get_figure() {
            Some(ref fig_pos) => {
                let fig = fig_pos.get_figure();
                if self.last_fig != *fig.get_name() {
                    self.last_fig = fig.get_name().clone();
                    self.handle_new_figure(fig_pos, pf);
                    self.last_path_update = current_ticks;
                }

                self.update_moves(current_ticks);
            }
            None => {}
        }
    }

    fn get_moves(&mut self, _: u64) -> Vec<(Movement, u64)> {
        let moves = self.moves.clone();
        self.moves.clear();
        return moves;
    }
}

impl ComputerPlayer {
    pub fn new(common: PlayerCommon, com_type: ComputerType) -> Self {
        ComputerPlayer {
            common: common,
            avail_pos: Vec::new(),
            last_fig: "".to_string(),
            last_path_update: 0,
            moves: Vec::new(),
            time_next_move: 0,
            com_type: com_type,
        }
    }

    fn handle_new_figure(&mut self, fig_pos: &FigurePos, pf: &Playfield) {
        let mut tmp_pf = pf.clone();
        fig_pos.remove(&mut tmp_pf);
        self.avail_pos =
            get_valid_placing(&tmp_pf, fig_pos);
        println!("New figure ({}) - {} available placings",
                 self.last_fig, self.avail_pos.len());
    }

    fn rand_move() -> Movement {
        let tmp = rand::random::<u8>() % 5;
        match tmp {
            0 => Movement::MoveLeft,
            1 => Movement::MoveRight,
            2 => Movement::MoveDown,
            3 => Movement::RotateCW,
            _ => Movement::RotateCCW,
        }
    }

    fn update_random(&mut self, current_ticks: u64) {
        self.moves.push((Self::rand_move(), current_ticks));
    }

    fn update_moves(&mut self, current_ticks: u64) {
        if self.time_next_move + 100000000 < current_ticks {
            self.time_next_move = current_ticks;
            match self.com_type {
                ComputerType::RandomStupid => self.update_random(current_ticks),
            }
        }
    }

}
