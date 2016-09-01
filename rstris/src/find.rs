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


fn distance(start: &Position, end: &Position) -> u32
{
    ((start.get_x() - end.get_x()).abs() +
     (start.get_y() - end.get_y()).abs() +
     (start.get_dir() - end.get_dir()).abs()) as u32
}

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    parent: usize,
    pos: Position,
    f: u32,
    g: u32,
    h: u32,
}

impl Node {
    fn new(node_list: &mut Vec<Node>, parent:  usize,
           pos: &Position, g: u32, h: u32)
           -> Node {
        let node = Node{id: node_list.len(),
                        parent: parent,
                        pos: pos.clone(),
                        g: g, h: h, f: g + h};
        node_list.push(node.clone());
        return node;
    }
}

pub fn find_path(pf: &Playfield, fig: &Figure,
                 start_pos: &Position, end_pos: &Position) -> Vec<Position>
{
    let mut all: Vec<Node> = Vec::new();
    let mut open_set: Vec<Node> = Vec::new();
    let mut closed_set: Vec<Node> = Vec::new();
    let start_node = Node::new(&mut all, 0, start_pos, 0, 0);
    open_set.push(start_node);

    while open_set.len() > 0 && all.len() < 10000 {
        open_set.sort_by(|a, b| b.f.cmp(&a.f));
        let q = open_set.pop().unwrap();

        // Find all possible movements from q (left,right,down,rotate)
        let mut successors: Vec<Node> = Vec::new();
        for movement in &[Movement::MoveLeft,
                          Movement::MoveRight,
                          Movement::MoveDown,
                          Movement::RotateCW] {

            let mut fig_pos = Position::apply_move(&q.pos, movement);
            fig_pos.normalize_dir(fig.get_num_dirs());
            if !fig.collide_blocked(pf, &fig_pos) {
                let succs = Node::new(&mut all, q.id, &fig_pos,
                                      q.g + distance(&q.pos, &fig_pos),
                                      distance(&fig_pos, end_pos));
                successors.push(succs);
            }
        }

        for s in successors {
            if s.pos == *end_pos {
                println!("Found path (from {:?} to {:?}):", start_pos, end_pos);
                let mut p = s;
                let mut path: Vec<Position> = Vec::new();
                path.push(p.pos.clone());
                while p.id != 0 {
                    p = all[p.parent].clone();
                    path.insert(0, p.pos.clone());
                }
                println!("Path: {:?}", path);
                return path;
            }
            if open_set.iter().find(
                |&n| n.pos == s.pos && n.f < s.f).is_none() &&
                closed_set.iter().find(
                    |&n| n.pos == s.pos && n.f < s.f).is_none() {
                open_set.push(s);
            }
        }
        closed_set.push(q);
    }
    return vec![];
}
