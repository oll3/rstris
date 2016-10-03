extern crate rand;

use rstris::playfield::*;
use rstris::position::*;
use rstris::figure_pos::*;
use rstris::find_placement::*;
use rstris::find_path::*;

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
    last_fig: String,
    avail_placing: Vec<EvalPosition>,
    last_path_update: u64,
    move_time: u64,
    path_per_height: Vec<Vec<MoveAndTime>>,
}


impl <'a>Player for ComputerPlayer<'a> {
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
        self.new_figure_event(pf, fig_pos, ticks);
        self.last_path_update = ticks;
    }
}

impl <'a> ComputerPlayer<'a> {
    pub fn new(common: PlayerCommon, move_time: u64,
               com_type: &'a mut ComputerType) -> Self {
        ComputerPlayer {
            common: common,
            avail_placing: Vec::new(),
            last_fig: "".to_string(),
            last_path_update: 0,
            move_time: move_time,
            com_type: com_type,
            path_per_height: Vec::new(),
        }
    }

    fn new_figure_event(&mut self, pf: &Playfield, fig_pos: &FigurePos,
                        ticks: u64) {
        let mut pf = pf.clone();
        fig_pos.remove(&mut pf);

        // Find all possible placings
        let avail_placing = find_placement_quick(&pf, fig_pos);
        println!("New figure ({}) - {} available placings",
                 self.last_fig, avail_placing.len());

        // Evaluate all placings to find the best one
        self.com_type.init_eval(&pf, self.avail_placing.len());
        let mut eval_placing: Vec<EvalPosition> = vec![];;
        for p in avail_placing {
            let eval_pos =
                FigurePos::new(fig_pos.get_figure().clone(), p.clone());
            let eval = self.com_type.eval_placing(&eval_pos, &pf);
            let eval_pos = EvalPosition{pos: p, eval: eval};
            eval_placing.push(eval_pos);
        }
        eval_placing.sort_by(|a, b| b.eval.cmp(&a.eval));
        self.avail_placing = eval_placing;

        // Find a path to first (and best) available placing
        let mut path = Vec::new();
        self.path_per_height.clear();
        for eval_pos in &self.avail_placing {
            path = find_path(&pf,
                             &fig_pos.get_figure(),
                             &fig_pos.get_position(),
                             &eval_pos.pos,
                             self.move_time,
                             self.common.force_down_time);
            if path.len() > 0 {
                break;
            }
        }
        if path.len() > 0 {
            path.reverse();

            // Convert the path from being in exact Movements to
            // describe the sideways/rotational movements per height level
            self.path_per_height = path_to_per_height(path);
            println!("Found path ({:?})", self.path_per_height);
        }
    }
}

fn path_to_per_height(path: Vec<(Movement, u64)>) -> Vec<Vec<MoveAndTime>> {
    let mut moves: Vec<Vec<MoveAndTime>> = Vec::new();
    let mut current_level: Vec<MoveAndTime> = Vec::new();
    for &(ref movement, time) in &path {
        if *movement == Movement::MoveDown {
            moves.push(current_level);
            current_level = Vec::new();
        }
        else {
            current_level.push(
                MoveAndTime{movement: movement.clone(), time: time}
            );
        }
    }
    return moves;
}
