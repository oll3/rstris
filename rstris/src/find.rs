extern crate time;

use std::collections::LinkedList;
use std::collections::HashSet;
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
                         fig_pos: &FigurePos) -> Vec<PosDir> {
    let current_ticks = time::precise_time_ns();
    let mut placements: Vec<PosDir> = Vec::new();
    let mut moves: LinkedList<PosDir> = LinkedList::new();
    let mut visited: HashSet<PosDir> = HashSet::new();
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
        let tmp_pos = PosDir::apply_move(&current_pos,
                                         &Movement::MoveLeft);
        if !visited.contains(&tmp_pos) && !fig.collide_locked(&pf, &tmp_pos) {
            visited.insert(tmp_pos.clone());
            moves.push_back(tmp_pos);
        }
        let tmp_pos = PosDir::apply_move(&current_pos,
                                         &Movement::MoveRight);
        if !visited.contains(&tmp_pos) && !fig.collide_locked(&pf, &tmp_pos) {
            visited.insert(tmp_pos.clone());
            moves.push_back(tmp_pos);
        }
        let tmp_pos = PosDir::apply_move(&current_pos,
                                         &Movement::RotateCW);
        if tmp_pos.get_dir() < fig.faces().len() as i32 &&
            !visited.contains(&tmp_pos) &&
            !fig.collide_locked(&pf, &tmp_pos) {
                visited.insert(tmp_pos.clone());
                moves.push_back(tmp_pos);
        }

        // Down is special. If we can't move down from current position then
        // the current position is a valid placement.
        let tmp_pos = PosDir::apply_move(&current_pos,
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


#[derive(Clone, Debug)]
struct Node {
    id: usize,
    parent: Option<usize>,
    pos: PosDir,
    walked_distance: u64, // g
    est_distance_end: u64, // h
    movement: Option<Movement>,
    time: u64,
    last_time_move: u64,
    last_time_down: u64,
}

impl Node {
    fn new(node_list: &mut Vec<Node>, parent: Option<usize>,
           pos: &PosDir, walked_distance: u64, est_distance_end: u64,
           movement: Option<Movement>,
           time: u64,
           last_time_move: u64,
           last_time_down: u64)
           -> Node {
        let node = Node{id: node_list.len(),
                        parent: parent,
                        pos: pos.clone(),
                        walked_distance: walked_distance,
                        est_distance_end: est_distance_end,
                        movement: movement,
                        time: time,
                        last_time_move: last_time_move,
                        last_time_down: last_time_down,
        };
        node_list.push(node.clone());
        return node;
    }

    fn get_tot_est(&self) -> u64 {
        self.walked_distance + self.est_distance_end
    }
}

use std::cmp::*;
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.parent == other.parent
    }
}

fn est_distance(start: &PosDir, end: &PosDir) -> u64
{
    ((start.get_x() - end.get_x()).abs() as u64 +
     (start.get_y() - end.get_y()).abs() as u64 +
     (start.get_dir() - end.get_dir()).abs() as u64)
}

fn no_pos_with_lower_est(set: &Vec<Node>, node: &Node) -> bool {
    set.iter().find(|&n| {
        n.pos == node.pos &&
            n.get_tot_est() < node.get_tot_est()}).is_none()
}

pub fn find_path(pf: &Playfield, fig: &Figure,
                 start_pos: &PosDir,
                 end_pos: &PosDir,
                 move_time: u64,
                 force_down_time: u64) -> Vec<(Movement, u64)>
{
    let mut all: Vec<Node> = Vec::new();
    let mut open_set: Vec<Node> = Vec::new();
    let mut closed_set: Vec<Node> = Vec::new();
    let start_node = Node::new(&mut all, None, start_pos, 0, 0, None, 0, 0, 0);
    open_set.push(start_node);

    println!("Find path {:?} -> {:?} (dist: {}, speed (move: {}, down: {})",
             start_pos, end_pos,
             est_distance(start_pos, end_pos),
             move_time, force_down_time);

    while open_set.len() > 0 && all.len() < 40000 {
        open_set.sort_by(|a, b| {
            b.get_tot_est().partial_cmp(&a.get_tot_est()).unwrap()
        });
        let q = open_set.pop().unwrap();

        // Decide if the next move is a forced down move or any move
        let time_since_move = (q.time - q.last_time_move) as i64;
        let time_since_down = (q.time - q.last_time_down) as i64;
        let time_until_move = move_time as i64 - time_since_move;
        let time_until_down = force_down_time as i64 - time_since_down;

        let movements;
        if time_until_move < time_until_down {
            // Regular move
            movements = vec![Movement::MoveLeft,
                             Movement::MoveRight,
                             Movement::MoveDown,
                             Movement::RotateCW];
        }
        else {
            // Forced down move
            movements = vec![Movement::MoveDown];
        }

        /*
        println!("Id: {}, p: {:?}, pos: {:?}, walked: {}, down: {}, move: {}",
                 q.id, q.parent, q.pos, q.walked_distance,
                 q.last_time_down, q.last_time_move); */

        // Find all possible movements from q (left,right,down,rotate)
        let mut successors: Vec<Node> = Vec::new();
        for movement in movements {

            let mut fig_pos = PosDir::apply_move(&q.pos, &movement);
            fig_pos.normalize_dir(fig.faces().len());
            if fig_pos != q.pos && !fig.collide_blocked(pf, &fig_pos) {
                let mut ltd = q.last_time_down;
                let mut ltm = q.last_time_move;
                let mut tt = q.time;
                if movement == Movement::MoveDown {
                    tt += max(0, time_until_down) as u64;
                    ltd = tt;
                }
                else {
                    tt += max(0, time_until_move) as u64;
                    ltm = tt;
                }
                let succs =
                    Node::new(&mut all, Some(q.id), &fig_pos,
                              q.walked_distance +
                              est_distance(&q.pos, &fig_pos),
                              est_distance(&fig_pos, end_pos),
                              Some(movement.clone()),
                              tt, ltm, ltd);
                successors.push(succs);
            }
        }

        for s in successors {
            if s.pos == *end_pos {
                // End was found
                let mut p = s;
                let mut path: Vec<(Movement, u64)> = Vec::new();
                while p.id != 0 {
                    path.push((p.movement.unwrap().clone(), p.time));
                    p = all[p.parent.unwrap()].clone();
                }
                return path;
            }

            if no_pos_with_lower_est(&open_set, &s) &&
                no_pos_with_lower_est(&closed_set, &s) {
                    open_set.push(s);
            }
        }
        closed_set.push(q);
    }
    println!("No path found for {} ({:?} to {:?} (distance: {}, tested: {})!",
             fig.get_name(), start_pos, end_pos,
             est_distance(start_pos, end_pos), all.len());
    return vec![];
}
