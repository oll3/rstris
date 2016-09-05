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


fn distance(start: &Position, end: &Position) -> u64
{
    ((start.get_x() - end.get_x()).abs() +
     (start.get_y() - end.get_y()).abs() +
     (start.get_dir() - end.get_dir()).abs()) as u64
}

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    parent: usize,
    pos: Position,
    f: u64,
    g: u64,
    h: u64,
    movement: Option<Movement>,
    total_time: u64,
    last_time_down: u64,
}

impl Node {
    fn new(node_list: &mut Vec<Node>, parent:  usize,
           pos: &Position, g: u64, h: u64,
           movement: Option<Movement>,
           total_time: u64,
           last_time_down: u64)
           -> Node {
        let node = Node{id: node_list.len(),
                        parent: parent,
                        pos: pos.clone(),
                        g: g, h: h, f: g + h,
                        movement: movement,
                        total_time: total_time,
                        last_time_down: last_time_down,
        };
        node_list.push(node.clone());
        return node;
    }
}

use std::cmp::*;
use std::hash::*;

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.parent == other.parent
    }
}
impl Eq for Node {}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.parent.hash(state);
    }
}

pub fn find_path(pf: &Playfield, fig: &Figure,
                 start_pos: &Position,
                 end_pos: &Position,
                 move_time: u64,
                 force_down_time: u64) -> Vec<(Movement, u64)>
{
    let mut all: Vec<Node> = Vec::new();
    let mut id: HashSet<Node> = HashSet::new();
    let mut open_set: Vec<Node> = Vec::new();
    let mut closed_set: Vec<Node> = Vec::new();
    let start_node = Node::new(&mut all, 0, start_pos, 0, 0, None, 0, 0);
    id.insert(start_node.clone());
    open_set.push(start_node);

    println!("Searching for path ({:?} to {:?} (distance: {})...",
             start_pos, end_pos, distance(start_pos, end_pos));
    while open_set.len() > 0 && all.len() < 200000 {
        open_set.sort_by(|a, b| b.f.partial_cmp(&a.f).unwrap());
        let q = open_set.pop().unwrap();

        let mut movements = vec![Movement::MoveLeft,
                                 Movement::MoveRight,
                                 Movement::MoveDown,
                                 Movement::RotateCW];

        let forced_move = (q.total_time - q.last_time_down) >= force_down_time;
        let mut succ_total_time = q.total_time + move_time;
        let mut last_time_down = q.last_time_down;

        println!("Id: {}, p: {}, pos: {:?}, time: {}, last down: {}, force: {}",
                 q.id, q.parent, q.pos, q.total_time,
                 q.last_time_down, forced_move);

/*        if forced_move {
            succ_total_time = q.total_time + force_down_time;
            movements = vec![Movement::MoveDown];
        }*/

        // Find all possible movements from q (left,right,down,rotate)
        let mut successors: Vec<Node> = Vec::new();
        for movement in movements {

            let mut fig_pos = Position::apply_move(&q.pos, &movement);
            fig_pos.normalize_dir(fig.get_num_dirs());
            if fig_pos != q.pos && !fig.collide_blocked(pf, &fig_pos) {
                let mut tmp = last_time_down;
                if movement == Movement::MoveDown {
                    println!("Increase next forced move time!");
                    tmp = succ_total_time;
                }
                let succs = Node::new(&mut all, q.id, &fig_pos,
                                      q.g + distance(&q.pos, &fig_pos),
                                      distance(&fig_pos, end_pos),
                                      Some(movement.clone()),
                                      succ_total_time,
                                      tmp);
                successors.push(succs);
            }
        }

        for s in successors {
            if s.pos == *end_pos {
                // End was found
                let mut p = s;
                let mut path: Vec<(Movement, u64)> = Vec::new();
                while p.id != 0 {
                    path.push((p.movement.unwrap().clone(), p.total_time));
                    p = all[p.parent].clone();
                }
                println!("Tested {} - Found path ({}): {:?}",
                         all.len(), path.len(), path);
                return path;
            }
            else if open_set.iter().find(
                |&n| n.pos == s.pos && n.f <= s.f).is_none() &&
                closed_set.iter().find(
                    |&n| n.pos == s.pos && n.f <= s.f).is_none() {

                    if id.contains(&s) {
                        println!("Already walked ({:?})?!", s);
                        return vec![];
                    }
                    id.insert(s.clone());
                    open_set.push(s);
            }
        }
        closed_set.push(q);
    }
    println!("No path found for {} ({:?} to {:?} (distance: {})!",
             fig.get_name(), start_pos, end_pos,
             distance(start_pos, end_pos));
    return vec![];
}
