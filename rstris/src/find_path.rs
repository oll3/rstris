extern crate time;

use std::collections::LinkedList;
use std::collections::HashSet;
use figure::*;
use figure_pos::*;
use position::*;
use playfield::*;


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
    let start_time = time::precise_time_ns();
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
                let search_time = (time::precise_time_ns() -
                                   start_time) as f64 / 1000000.0;
                println!("Path found for {} ({:?} to {:?} ({} ms)",
                         fig.get_name(), start_pos, end_pos, search_time);
                return path;
            }

            if no_pos_with_lower_est(&open_set, &s) &&
                no_pos_with_lower_est(&closed_set, &s) {
                    open_set.push(s);
            }
        }
        closed_set.push(q);
    }
    let search_time = (time::precise_time_ns() -
                       start_time) as f64 / 1000000.0;
    println!("No path found for {} ({:?} to {:?} (distance: {}, tested: {}, {} ms)!",
             fig.get_name(), start_pos, end_pos,
             est_distance(start_pos, end_pos), all.len(), search_time);
    return vec![];
}