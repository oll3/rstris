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
    path: Vec<(Movement, u64)>
}

impl Player for ComputerPlayer {
    fn common(&self) -> &PlayerCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut PlayerCommon {
        &mut self.common
    }

    fn handle_new_figure(&mut self, current_ticks: u64,
                         pf: &Playfield, fig_pos: &FigurePos) {
        self.handle_new_figure(pf, fig_pos);
        self.last_path_update = current_ticks;
    }

    fn update(&mut self, current_ticks: u64, pf: &Playfield) {
        match self.common.get_figure() {
            Some(ref fig_pos) => self.update_moves(current_ticks),
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
            path: Vec::new(),
        }
    }

    fn handle_new_figure(&mut self, pf: &Playfield, fig_pos: &FigurePos) {
        self.avail_pos = get_valid_placing(&pf, fig_pos);
        println!("New figure ({}) - {} available placings",
                 self.last_fig, self.avail_pos.len());

        self.avail_pos.sort_by(|a, b| b.get_y().cmp(&a.get_y()));
        let sel_end = 0;
        self.path = find_path(&pf,
                              &fig_pos.get_figure(),
                              &fig_pos.get_position(),
                              &self.avail_pos[sel_end],
                              self.common.force_down_time/2,
                              self.common.force_down_time);
        self.path.insert(0, (Movement::MoveDown, 0));
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

    fn update_moves(&mut self, current_ticks: u64) {
        if self.path.len() > 0 {
            let (movement, time) = self.path[self.path.len()-1].clone();
            if (current_ticks - self.last_path_update) > time {
                self.moves.push(self.path.pop().unwrap());
            }
        }
    }
}
