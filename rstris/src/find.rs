extern crate time;

use std::collections::LinkedList;
use std::collections::HashSet;
use std::collections::HashMap;
use figure::*;
use figure_pos::*;
use position::*;
use playfield::*;


//
// Returns a list of all valid placements of a figure in
// playfield. All positions returned should be possible to
// reach from the starting point.
//
pub fn get_valid_placing(pf: &Playfield,
                         fig_pos: &FigurePos) -> Vec<Position> {
    let current_ticks = time::precise_time_ns();
    let mut placements: Vec<Position> = Vec::new();
    let mut moves: LinkedList<Position> = LinkedList::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut it_cnt = 0;
    let start_pos = fig_pos.get_position().clone();
    let fig = fig_pos.get_figure();

    println!("Find valid placements for figure {} (starting at {:?})",
             fig.get_name(), start_pos);
    if fig.collide_locked(pf, &start_pos) {
        println!("Invalid starting point ({:?}) for figure {}",
                 start_pos, fig.get_name());
        return placements;
    }

    visited.insert(start_pos.clone());
    moves.push_back(start_pos);

    while moves.len() > 0 {
        let current_pos = moves.pop_front().unwrap();

        // Visist all the closest positions that has not been visited
        // already (one left, right, down, rotate cw).
        let tmp_pos = Position::apply_move(&current_pos,
                                           &Movement::MoveLeft);
        if !visited.contains(&tmp_pos) && !fig.collide_locked(&pf, &tmp_pos) {
            visited.insert(tmp_pos.clone());
            moves.push_back(tmp_pos);
        }
        let tmp_pos = Position::apply_move(&current_pos,
                                           &Movement::MoveRight);
        if !visited.contains(&tmp_pos) && !fig.collide_locked(&pf, &tmp_pos) {
            visited.insert(tmp_pos.clone());
            moves.push_back(tmp_pos);
        }
        let tmp_pos = Position::apply_move(&current_pos,
                                           &Movement::RotateCW);
        if tmp_pos.get_dir() < fig.get_num_dirs() as i32 &&
            !visited.contains(&tmp_pos) &&
            !fig.collide_locked(&pf, &tmp_pos) {
                visited.insert(tmp_pos.clone());
                moves.push_back(tmp_pos);
        }

        // Down is special. If we can't move down from current position then
        // the current position is a valid placement.
        let tmp_pos = Position::apply_move(&current_pos,
                                           &Movement::MoveDown);
        if fig.collide_locked(&pf, &tmp_pos) {
            // Valid placement
            println!("Valid position: {:?}", tmp_pos);
            placements.push(current_pos.clone());
        } else if !visited.contains(&tmp_pos) {
            moves.push_back(tmp_pos.clone());
            visited.insert(tmp_pos);
        }
        it_cnt += 1;
    }
    println!("Found {} valid placements for {} (iterated {} times, {} ms)",
             placements.len(), fig.get_name(), it_cnt,
             (time::precise_time_ns() - current_ticks) as f64 / 1000000.0);
    return placements;
}


pub fn cost_estimate(start: &Position, end: &Position) -> u32
{
    ((start.get_x() - end.get_x()).abs() +
     (start.get_y() - end.get_y()).abs() +
     (start.get_dir() - end.get_dir()).abs()) as u32
}

fn get_score(pos: &Position, pos_score: &HashMap<Position, u32>) -> u32
{
    match pos_score.get(pos) {
        Some(score) => *score,
        None => 1000,
    }
}

pub fn find_path_astar(pf: &Playfield, fig: &Figure,
                       start_pos: &Position, end_pos: &Position)
{
    let mut closed_set: HashSet<Position> = HashSet::new();
    let mut open_set: Vec<Position> = Vec::new();
    open_set.push(start_pos.clone());
    let mut cost_to_pos: HashMap<Position, u32> = HashMap::new();
    let mut pos_score: HashMap<Position, u32> = HashMap::new();
    cost_to_pos.insert(start_pos.clone(), 0);
    pos_score.insert(start_pos.clone(),
                     cost_estimate(&start_pos, &end_pos));

    while open_set.len() > 0 {
        if open_set.len() > 1 {
            open_set.sort_by(|a, b|
                             get_score(&a, &pos_score).
                             cmp(&get_score(&b, &pos_score)))
        }

    }
}
