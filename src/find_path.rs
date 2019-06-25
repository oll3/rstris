use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::figure::*;
use crate::matrix3::Matrix3;
use crate::movement::*;
use crate::playfield::*;
use crate::pos_dir::*;
use crate::vec3::Vec3;

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

#[derive(Debug)]
struct NodeContext {
    pf: Playfield,
    fig: Figure,
    end_pos: PosDir,
    moves_per_down_step: f32,

    // Node by id contains all created nodes, indexed by id
    node_by_id: Vec<Node>,

    // Node by position is used to make the search for already
    // visited positions quicker.
    node_by_pos: Matrix3<Option<usize>>,

    open_set: BinaryHeap<NodeIdAndEst>,
}

impl NodeContext {
    fn new(moves_per_down_step: f32, pf: &Playfield, fig: &Figure, end_pos: &PosDir) -> Self {
        NodeContext {
            pf: pf.clone(),
            fig: fig.clone(),
            end_pos: *end_pos,
            moves_per_down_step,
            node_by_id: Vec::new(),
            node_by_pos: Matrix3::new_coords(
                Vec3::new((-(fig.max_width() as i32), -(fig.max_width() as i32), 0)),
                Vec3::new((
                    (pf.width() + fig.max_width()) as i32,
                    (pf.height() + fig.max_width()) as i32,
                    fig.faces().len() as i32,
                )),
                None,
            ),
            open_set: BinaryHeap::new(),
        }
    }
    fn get_node_from_id(&self, id: usize) -> &Node {
        &self.node_by_id[id]
    }
    fn pop_best_open(&mut self) -> Node {
        let best_node = self.open_set.pop().unwrap();
        self.get_node_from_id(best_node.id).clone()
    }
    fn mark_best_pos(&mut self, node: &Node) {
        self.node_by_pos.set(node.pos, Some(node.id));
    }
    fn add_node(&mut self, node: Node) {
        self.node_by_id.push(node);
    }
    fn mark_open(&mut self, id_and_est: NodeIdAndEst) {
        self.open_set.push(id_and_est);
    }
    fn mark_closed(&mut self, _: &Node) {}
    fn no_pos_with_lower_est(&self, node: &Node) -> bool {
        if let Some(best_node) = *self.node_by_pos.get(node.pos) {
            let n = self.get_node_from_id(best_node);
            if n.id != node.id && n.get_tot_est() <= node.get_tot_est() {
                return false;
            }
        }
        true
    }
    fn process_and_test_for_end(&mut self, end_pos: &PosDir, node_id: usize) -> bool {
        let node = &self.node_by_id[node_id];
        let id_and_est = node.get_id_and_est();
        if node.pos == *end_pos {
            return true;
        } else if !self.fig.test_collision(&self.pf, &node.pos) && self.no_pos_with_lower_est(node)
        {
            self.node_by_pos.set(node.pos, Some(node_id));
            self.mark_open(id_and_est);
        }
        false
    }
}

fn est_pos_distance(start: &PosDir, end: &PosDir) -> u64 {
    ((start.get_x() - end.get_x()).abs() as u64 + (start.get_dir() - end.get_dir()).abs() as u64)
}

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    parent_id: Option<usize>,
    pos: PosDir,
    walked: u64,  // g
    est_end: u64, // h
    movement: Option<Movement>,
    move_count: f32,
}

impl Node {
    fn new(
        id: usize,
        parent_id: Option<usize>,
        pos: &PosDir,
        walked: u64,
        est_distance_end: u64,
        movement: Option<Movement>,
        move_count: f32,
    ) -> Self {
        Node {
            id,
            parent_id,
            pos: *pos,
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

    fn new_moved_node(&self, ctx: &mut NodeContext, movement: Movement, move_count: f32) -> usize {
        let mut fig_pos = PosDir::apply_move(&self.pos, movement);
        fig_pos.normalize_dir(ctx.fig.faces().len());

        let distance_to_end = est_pos_distance(&fig_pos, &ctx.end_pos);
        let node_id = ctx.node_by_id.len();
        let node = Node::new(
            node_id,
            Some(self.id),
            &fig_pos,
            self.walked + 1,
            distance_to_end,
            Some(movement),
            move_count,
        );
        ctx.add_node(node);
        node_id
    }

    fn get_possible_moves(&self, moves: &mut Vec<usize>, ctx: &mut NodeContext) {
        if self.move_count <= 0.0 {
            // We're allowed to move in any direction
            let new_move_count = self.move_count + 1.0 / ctx.moves_per_down_step;
            moves.push(self.new_moved_node(ctx, Movement::MoveLeft, new_move_count));
            moves.push(self.new_moved_node(ctx, Movement::MoveRight, new_move_count));
            moves.push(self.new_moved_node(ctx, Movement::RotateCW, new_move_count));
            moves.push(self.new_moved_node(ctx, Movement::RotateCCW, new_move_count));
        }
        moves.push(self.new_moved_node(ctx, Movement::MoveDown, self.move_count - 1.0));
    }

    fn get_path(&self, path: &mut Vec<Movement>, ctx: &NodeContext) {
        let mut p = self;
        while p.id != 0 {
            if let Some(ref movement) = p.movement {
                path.push(*movement);
            }
            p = &ctx.node_by_id[p.parent_id.unwrap()];
        }
    }
}

#[derive(Default)]
pub struct FindPath {
    possible_nodes: Vec<usize>,
}
impl FindPath {
    pub fn search(
        &mut self,
        path: &mut Vec<Movement>,
        pf: &Playfield,
        fig: &Figure,
        start_pos: &PosDir,
        end_pos: &PosDir,
        moves_per_down_step: f32,
    ) {
        let mut ctx = NodeContext::new(moves_per_down_step, pf, fig, end_pos);
        let start_node = Node::new(
            ctx.node_by_id.len(),
            None,
            start_pos,
            0,
            est_pos_distance(start_pos, end_pos),
            None,
            0.0,
        );
        ctx.add_node(start_node.clone());
        ctx.mark_open(start_node.get_id_and_est());
        ctx.mark_best_pos(&start_node);
        path.clear();

        self.possible_nodes.clear();
        while !ctx.open_set.is_empty() {
            let q = ctx.pop_best_open();

            q.get_possible_moves(&mut self.possible_nodes, &mut ctx);
            for node_id in &self.possible_nodes {
                if ctx.process_and_test_for_end(end_pos, *node_id) {
                    // End was found - Reconstruct path from end node
                    ctx.get_node_from_id(*node_id).get_path(path, &ctx);
                    return;
                }
                /*
                let node = ctx.get_node_from_id(*node_id);
                let id_and_est = node.get_id_and_est();
                if node.pos == *end_pos {
                    // End was found - Reconstruct path from end node
                    node.get_path(path, &ctx);
                    return;
                } else if !fig.test_collision(&ctx.pf, &node.pos) && ctx.no_pos_with_lower_est(node)
                {
                    ctx.mark_best_pos(node);
                    ctx.mark_open(id_and_est);
                }
                */
            }
            self.possible_nodes.clear();
            ctx.mark_closed(&q);
        }
        // No path found
    }
}
