use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::figure::Figure;
use crate::matrix3::Matrix3;
use crate::movement::Movement;
use crate::playfield::Playfield;
use crate::position::Position;

#[derive(Debug, Clone, Copy)]
struct NodeIdAndEst {
    id: usize,
    est: u64,
}
impl Ord for NodeIdAndEst {
    fn cmp(&self, other: &NodeIdAndEst) -> Ordering {
        other.est.cmp(&self.est)
    }
}
impl PartialOrd for NodeIdAndEst {
    fn partial_cmp(&self, other: &NodeIdAndEst) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for NodeIdAndEst {}
impl PartialEq for NodeIdAndEst {
    fn eq(&self, other: &NodeIdAndEst) -> bool {
        self.est == other.est
    }
}

fn est_pos_distance(start: Position, end: Position) -> u64 {
    ((start.x() - end.x()).abs() as u64 + (start.dir() - end.dir()).abs() as u64)
}

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    parent_id: Option<usize>,
    pos: Position,
    walked: u64,  // g
    est_end: u64, // h
    movement: Option<Movement>,
    move_count: f32,
}

impl Node {
    fn new(
        id: usize,
        parent_id: Option<usize>,
        pos: Position,
        walked: u64,
        est_distance_end: u64,
        movement: Option<Movement>,
        move_count: f32,
    ) -> Self {
        Node {
            id,
            parent_id,
            pos,
            walked,
            est_end: est_distance_end,
            movement,
            move_count,
        }
    }

    fn get_tot_est(&self) -> u64 {
        self.walked + self.est_end
    }

    fn get_id_and_est(&self) -> NodeIdAndEst {
        NodeIdAndEst {
            id: self.id,
            est: self.get_tot_est(),
        }
    }

    fn get_path(&self, path: &mut Vec<Movement>, ctx: &FindPath) {
        let mut p = self;
        while p.id != 0 {
            if let Some(ref movement) = p.movement {
                path.push(*movement);
            }
            p = &ctx.node_by_id[p.parent_id.unwrap()];
        }
    }
}

struct NodeByPos {
    pf_width: i32,
    pf_height: i32,
    max_fig_width: i32,
    m: Matrix3<Option<usize>>,
}

impl NodeByPos {
    fn new(pf_width: i32, pf_height: i32, max_fig_width: i32) -> Self {
        Self {
            pf_width,
            pf_height,
            max_fig_width,
            m: Matrix3::new_coords(
                Position::new((-max_fig_width, -max_fig_width, 0)),
                Position::new((pf_width + max_fig_width, pf_height + max_fig_width, 4)),
                None,
            ),
        }
    }

    fn resize_and_clear(&mut self, pf_width: i32, pf_height: i32, max_fig_width: i32) {
        if pf_width > self.pf_width
            || pf_height > self.pf_height
            || max_fig_width > self.max_fig_width
        {
            self.pf_width = pf_width;
            self.pf_height = pf_height;
            self.max_fig_width = max_fig_width;
            self.m = Matrix3::new_coords(
                Position::new((-max_fig_width, -max_fig_width, 0)),
                Position::new((pf_width + max_fig_width, pf_height + max_fig_width, 4)),
                None,
            );
        } else {
            self.m.fill(None);
        }
    }
}

pub struct FindPath {
    possible_nodes: Vec<usize>,
    end_pos: Position,
    moves_per_down_step: f32,
    fig_faces: u8,
    // Node by id contains all created nodes, indexed by id
    node_by_id: Vec<Node>,

    // Node by position is used to make the search for already
    // visited positions quicker.
    node_by_pos: NodeByPos,

    open_set: BinaryHeap<NodeIdAndEst>,
}

impl Default for FindPath {
    fn default() -> Self {
        Self::new()
    }
}

impl FindPath {
    // Create new find path cache
    pub fn new() -> Self {
        Self {
            possible_nodes: Vec::new(),
            end_pos: Position::new((0, 0, 0)),
            fig_faces: 0,
            moves_per_down_step: 0.0,
            node_by_id: Vec::new(),
            node_by_pos: NodeByPos::new(0, 0, 0),
            open_set: BinaryHeap::new(),
        }
    }

    // Prepare the cache for new search
    fn prepare(
        &mut self,
        moves_per_down_step: f32,
        pf: &Playfield,
        fig: &Figure,
        end_pos: Position,
    ) {
        self.end_pos = end_pos;
        self.fig_faces = fig.num_faces();
        self.moves_per_down_step = moves_per_down_step;
        self.node_by_id.clear();
        self.open_set.clear();
        self.node_by_pos.resize_and_clear(
            pf.width() as i32,
            pf.height() as i32,
            i32::from(fig.max_width()),
        );
    }
    fn new_moved_node(&mut self, node_id: usize, movement: Movement, move_count: f32) {
        let node = &self.node_by_id[node_id];
        let mut fig_pos = Position::apply_move(&node.pos, movement);
        fig_pos.normalize_dir(self.fig_faces);

        let distance_to_end = est_pos_distance(fig_pos, self.end_pos);
        let node_id = self.node_by_id.len();
        let node = Node::new(
            node_id,
            Some(node.id),
            fig_pos,
            node.walked + 1,
            distance_to_end,
            Some(movement),
            move_count,
        );
        self.node_by_id.push(node);
        self.possible_nodes.push(node_id);
    }

    fn get_possible_moves(&mut self, node_id: usize) {
        self.possible_nodes.clear();
        if self.node_by_id[node_id].move_count <= 0.0 {
            // We're allowed to move in any direction
            let new_move_count =
                self.node_by_id[node_id].move_count + 1.0 / self.moves_per_down_step;
            for movement in &[
                Movement::MoveLeft,
                Movement::MoveRight,
                Movement::RotateCW,
                Movement::RotateCCW,
            ] {
                self.new_moved_node(node_id, *movement, new_move_count);
            }
        }
        self.new_moved_node(
            node_id,
            Movement::MoveDown,
            self.node_by_id[node_id].move_count - 1.0,
        );
    }

    fn get_node_from_id(&self, id: usize) -> &Node {
        &self.node_by_id[id]
    }
    fn pop_best_open(&mut self) -> usize {
        let best_node = self.open_set.pop().unwrap();
        best_node.id
    }
    fn no_pos_with_lower_est(&self, node: &Node) -> bool {
        if let Some(best_node) = *self.node_by_pos.m.get(node.pos) {
            let n = self.get_node_from_id(best_node);
            if n.id != node.id && n.get_tot_est() <= node.get_tot_est() {
                return false;
            }
        }
        true
    }

    pub fn search(
        &mut self,
        path: &mut Vec<Movement>,
        pf: &Playfield,
        fig: &Figure,
        start_pos: Position,
        end_pos: Position,
        moves_per_down_step: f32,
    ) {
        self.prepare(moves_per_down_step, pf, fig, end_pos);
        let start_node = Node::new(
            self.node_by_id.len(),
            None,
            start_pos,
            0,
            est_pos_distance(start_pos, end_pos),
            None,
            0.0,
        );

        self.node_by_id.push(start_node.clone());
        self.open_set.push(start_node.get_id_and_est());
        self.node_by_pos.m.set(start_node.pos, Some(start_node.id));
        path.clear();

        while !self.open_set.is_empty() {
            let best_node_id = self.pop_best_open();
            self.get_possible_moves(best_node_id);
            for node_id in &self.possible_nodes {
                let node = &self.node_by_id[*node_id];
                let id_and_est = node.get_id_and_est();
                if node.pos == end_pos {
                    self.get_node_from_id(*node_id).get_path(path, self);
                    return;
                } else if !fig.test_collision(pf, node.pos) && self.no_pos_with_lower_est(node) {
                    self.node_by_pos.m.set(node.pos, Some(*node_id));
                    self.open_set.push(id_and_est);
                }
            }
        }
        // No path found
    }
}
