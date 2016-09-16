extern crate rand;

use rstris::playfield::*;
use rstris::position::*;
use rstris::figure_pos::*;
use rstris::find_placement::*;
use rstris::find::*;

use player::*;

pub trait ComputerType {
    fn init_eval(&mut self, pf: &Playfield, avail_placings: usize) {}
    fn eval_placing(&mut self, figure_pos: &FigurePos, pf: &Playfield) -> i32;
}

struct EvalPosition {
    pos: PosDir,
    eval: i32,
}

pub struct ComputerPlayer<'a> {
    common: PlayerCommon,
    com_type: &'a mut ComputerType,
    moves: Vec<(Movement, u64)>,
    last_fig: String,
    avail_pos: Vec<EvalPosition>,
    last_path_update: u64,
    move_time: u64,
    path: Vec<(Movement, u64)>
}


impl <'a>Player for ComputerPlayer<'a> {
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

impl <'a> ComputerPlayer<'a> {
    pub fn new(common: PlayerCommon, move_time: u64,
               com_type: &'a mut ComputerType) -> Self {
        ComputerPlayer {
            common: common,
            avail_pos: Vec::new(),
            last_fig: "".to_string(),
            last_path_update: 0,
            moves: Vec::new(),
            move_time: move_time,
            com_type: com_type,
            path: Vec::new(),
        }
    }

    fn handle_new_figure(&mut self, pf: &Playfield, fig_pos: &FigurePos) {
        let avail_placing = find_placement(&pf, fig_pos);
        println!("New figure ({}) - {} available placings",
                 self.last_fig, avail_placing.len());
        self.com_type.init_eval(pf, self.avail_pos.len());
        let mut eval_placing: Vec<EvalPosition> = vec![];;
        for p in avail_placing {
            let eval_pos =
                FigurePos::new(fig_pos.get_figure().clone(), p.clone());
            let eval = self.com_type.eval_placing(&eval_pos, pf);
            let eval_pos = EvalPosition{pos: p, eval: eval};
            eval_placing.push(eval_pos);
        }
        eval_placing.sort_by(|a, b| b.eval.cmp(&a.eval));
        self.avail_pos = eval_placing;
        let sel_end = 0;
        for eval_pos in &self.avail_pos {
            let path = find_path(&pf,
                                 &fig_pos.get_figure(),
                                 &fig_pos.get_position(),
                                 &eval_pos.pos,
                                 self.move_time,
                                 self.common.force_down_time);
            if path.len() > 0 {
                self.path = path;
                self.path.insert(0, (Movement::MoveDown, 0));
                break;
            }
        }
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
