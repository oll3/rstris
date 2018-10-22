extern crate rand;

use rstris::figure_pos::*;
use rstris::find_path::*;
use rstris::find_placement::*;
use rstris::movement::*;
use rstris::playfield::*;
use rstris::pos_dir::*;

use player::*;

pub trait ComputerType {
    fn init_eval(&mut self, pf: &Playfield, avail_placings: usize);
    fn eval_placing(&mut self, figure_pos: &FigurePos, pf: &Playfield) -> f32;
}

struct EvalPosition {
    pos: PosDir,
    eval: f32,
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

impl<'a> Player for ComputerPlayer<'a> {
    fn common(&self) -> &PlayerCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut PlayerCommon {
        &mut self.common
    }

    fn figure_move_event(
        &mut self,
        ticks: u64,
        _: &Playfield,
        fig_pos: &FigurePos,
        movement: &Movement,
    ) {
        let y = fig_pos.get_position().get_y() as usize;
        if *movement == Movement::MoveDown && y < self.path_per_height.len() {
            let moves = self.path_per_height[y].clone();
            let time_between_moves = self.common().force_down_time / (moves.len() + 1) as u64;
            let mut movement_time = ticks;
            for move_and_time in &moves {
                movement_time += time_between_moves;
                self.common_mut()
                    .add_move(move_and_time.movement.clone(), movement_time);
            }
        }
    }

    fn new_figure_event(&mut self, ticks: u64, pf: &Playfield, fig_pos: &FigurePos) {
        let mut pf_no_fig = pf.clone();
        fig_pos.remove(&mut pf_no_fig);

        // Find all possible placings
        let avail_placing = find_placement_quick(&pf_no_fig, fig_pos);
        println!(
            "New figure ({}) - {} available placings",
            self.last_fig,
            avail_placing.len()
        );

        // Evaluate all placings to find the best one
        self.com_type
            .init_eval(&pf_no_fig, self.avail_placing.len());
        let mut eval_placing: Vec<EvalPosition> = vec![];
        for p in avail_placing {
            let eval_pos = FigurePos::new(fig_pos.get_figure().clone(), p.clone());
            let eval = self.com_type.eval_placing(&eval_pos, &pf_no_fig);
            let eval_pos = EvalPosition { pos: p, eval: eval };
            eval_placing.push(eval_pos);
        }
        eval_placing.sort_by(|a, b| b.eval.partial_cmp(&a.eval).unwrap());
        self.avail_placing = eval_placing;

        // Find a path to first (and best) available placing
        let mut path = Vec::new();
        for eval_pos in &self.avail_placing {
            path = find_path(
                &pf_no_fig,
                &fig_pos.get_figure(),
                &fig_pos.get_position(),
                &eval_pos.pos,
                self.move_time,
                self.common.force_down_time,
            );
            if path.len() > 0 {
                break;
            }
        }

        self.path_per_height.clear();
        if path.len() > 0 {
            path.reverse();

            // Convert the path from being in exact Movements to
            // describe the sideways/rotational movements per height level
            self.path_per_height = path_to_per_height(path);
        }

        self.last_path_update = ticks;
        self.figure_move_event(ticks, pf, fig_pos, &Movement::MoveDown);
    }
}

impl<'a> ComputerPlayer<'a> {
    pub fn new(common: PlayerCommon, move_time: u64, com_type: &'a mut ComputerType) -> Self {
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
}

fn path_to_per_height(path: Vec<(Movement, u64)>) -> Vec<Vec<MoveAndTime>> {
    let mut moves: Vec<Vec<MoveAndTime>> = Vec::new();
    let mut current_level: Vec<MoveAndTime> = Vec::new();
    for &(ref movement, time) in &path {
        if *movement == Movement::MoveDown {
            moves.push(current_level);
            current_level = Vec::new();
        } else {
            current_level.push(MoveAndTime {
                movement: movement.clone(),
                time: time,
            });
        }
    }
    if current_level.len() > 0 {
        moves.push(current_level);
    }
    return moves;
}
