extern crate time;

use std::collections::HashMap;
use std::cmp::Ordering;
use std::cmp::max;
use figure::*;
use position::*;
use playfield::*;

#[derive(Debug)]
struct NodeContext {
    pf: Playfield,
    fig: Figure,
    end_pos: PosDir,
    start_time: u64,
    move_time: u64,
    down_time: u64,

    // Node by id contains all created nodes, indexed by id
    node_by_id: Vec<Node>,

    // Node by position is used to make the search for already
    // visited positions quicker.
    node_by_pos: HashMap<PosDir, Vec<usize>>,

    open_set: Vec<usize>,
    closed_set: Vec<usize>,
}

impl NodeContext {
    fn new(move_time: u64, down_time: u64,
           pf: &Playfield,
           fig: &Figure,
           end_pos: &PosDir) -> Self {
        NodeContext {
            pf: pf.clone(),
            fig: fig.clone(),
            end_pos: end_pos.clone(),
            start_time: time::precise_time_ns(),
            move_time: move_time,
            down_time: down_time,
            node_by_id: Vec::new(),
            node_by_pos: HashMap::new(),
            open_set: Vec::new(),
            closed_set: Vec::new(),
        }
    }
    fn get_node_from_id(&self, id: usize) -> &Node {
        return &self.node_by_id[id];
    }
    fn pop_best_open(&mut self) -> Node {
        let mut sorted_list = self.open_set.clone();
        sorted_list.sort_by(|a, b| {
            Node::cmp_est(self.get_node_from_id(*b),
                          self.get_node_from_id(*a)).unwrap()
        });
        self.open_set = sorted_list;
        let best_node_id = self.open_set.pop().unwrap();
        return self.get_node_from_id(best_node_id).clone();
    }
    fn add_by_pos_ref(&mut self, node: &Node) {
        let pos_list =
            self.node_by_pos.entry(node.pos.clone()).or_insert(Vec::new());
        pos_list.push(node.id);
    }
    fn add_node(&mut self, node: Node) {
        self.add_by_pos_ref(&node);
        self.node_by_id.push(node);
    }
    fn mark_open(&mut self, node: &Node) {
        self.open_set.push(node.id);
    }
    fn mark_closed(&mut self, node: &Node) {
        self.closed_set.push(node.id);
    }
    fn no_pos_with_lower_est(&self, node: &Node) -> bool {
        if let Some(pos_list) = self.node_by_pos.get(&node.pos) {
            for id in pos_list {
                let n = self.get_node_from_id(*id);
                if n.id != node.id && n.get_tot_est() <= node.get_tot_est() {
                    return false;
                }
            }
        }
        return true;
    }
}


fn est_pos_distance(start: &PosDir, end: &PosDir) -> u64 {
    ((start.get_x() - end.get_x()).abs() as u64 +
     (start.get_dir() - end.get_dir()).abs() as u64)
}


#[derive(Clone, Debug)]
struct Node {
    id: usize,
    pid: Option<usize>,
    pos: PosDir,
    walked: u64, // g
    est_end: u64, // h
    mvmnt: Option<Movement>,
    time: u64,
    last_time_move: u64,
    last_time_down: u64,
}

impl Node {
    fn new(ctx: &mut NodeContext,
           parent: Option<usize>,
           pos: &PosDir, walked: u64, est_distance_end: u64,
           mvmnt: Option<Movement>, time: u64,
           last_time_move: u64, last_time_down: u64)
           -> Node {

        let node = Node{
            id: ctx.node_by_id.len(),
            pid: parent,
            pos: pos.clone(),
            walked: walked,
            est_end: est_distance_end,
            mvmnt: mvmnt,
            time: time,
            last_time_move: last_time_move,
            last_time_down: last_time_down,
        };
        ctx.add_node(node.clone());
        return node;
    }

    fn get_tot_est(&self) -> u64 {
        self.walked + self.est_end
    }

    fn cmp_est(n1: &Node, n2: &Node) -> Option<Ordering> {
        n1.get_tot_est().partial_cmp(&n2.get_tot_est())
    }

    fn time_until_move(&self, ctx: &NodeContext) -> i64 {
        let time_since_move = (self.time - self.last_time_move) as i64;
        return ctx.move_time as i64 - time_since_move;
    }
    fn time_until_down(&self, ctx: &NodeContext) -> i64 {
        let time_since_down = (self.time - self.last_time_down) as i64;
        return ctx.down_time as i64 - time_since_down;
    }

    fn new_moved_node(&self, ctx: &mut NodeContext,
                      movement: Movement) -> Node {
        let mut fig_pos = PosDir::apply_move(&self.pos, &movement);
        fig_pos.normalize_dir(ctx.fig.faces().len());

        let mut ltd = self.last_time_down;
        let mut ltm = self.last_time_move;
        let mut tt = self.time;

        if movement == Movement::MoveDown {
            tt += max(0, self.time_until_down(ctx)) as u64;
            ltd = tt;
        }
        else {
            tt += max(0, self.time_until_move(ctx)) as u64;
            ltm = tt;
        }

        let distance_to_end = est_pos_distance(&fig_pos, &ctx.end_pos);
        Node::new(ctx, Some(self.id), &fig_pos,
                  self.walked + 1,
                  distance_to_end,
                  Some(movement.clone()),
                  tt, ltm, ltd)
    }

    fn get_possible_moves(&self, ctx: &mut NodeContext) -> Vec<Node> {

        if self.time_until_move(ctx) < self.time_until_down(ctx) {
            // Regular move
            return vec![self.new_moved_node(ctx, Movement::MoveLeft),
                        self.new_moved_node(ctx, Movement::MoveRight),
                        self.new_moved_node(ctx, Movement::RotateCW),
                        self.new_moved_node(ctx, Movement::RotateCCW),
                        self.new_moved_node(ctx, Movement::MoveDown)];
        }
        else {
            // Forced down move
            return vec![self.new_moved_node(ctx, Movement::MoveDown)];
        }
    }

    fn get_path(&self, ctx: &NodeContext) -> Vec<(Movement, u64)> {
        let mut p = self;
        let mut path: Vec<(Movement, u64)> = Vec::new();
        while p.id != 0 {
            path.push((p.mvmnt.clone().unwrap(), p.time));
            p = &ctx.node_by_id[p.pid.unwrap()];
        }
        return path;
    }
}

pub fn find_path(pf: &Playfield, fig: &Figure,
                 start_pos: &PosDir,
                 end_pos: &PosDir,
                 move_time: u64,
                 force_down_time: u64) -> Vec<(Movement, u64)>
{
    let mut ctx = NodeContext::new(move_time,
                                   force_down_time,
                                   pf, fig, end_pos);
    let start_node = Node::new(&mut ctx, None, start_pos, 0,
                               est_pos_distance(start_pos, end_pos),
                               None, 0, 0, 0);
    ctx.mark_open(&start_node);

    println!("Find path {:?} -> {:?} (dist: {}, speed (move: {}, down: {})",
             start_pos, end_pos,
             est_pos_distance(start_pos, end_pos),
             move_time, force_down_time);

    while ctx.open_set.len() > 0 {
        let q = ctx.pop_best_open();

        let possible_nodes = q.get_possible_moves(&mut ctx);
        for node in possible_nodes {
            if node.pos == *end_pos {
                // End was found
                let search_time =
                    (time::precise_time_ns() - ctx.start_time) as f64/1000000.0;
                println!("Path found for {} in {} ms (searched {} nodes)",
                         ctx.fig.get_name(), search_time, ctx.node_by_id.len());

                // Reconstruct path from end node
                return node.get_path(&ctx);
            }
            else if !fig.collide_blocked(&ctx.pf, &node.pos) &&
                ctx.no_pos_with_lower_est(&node)
            {
                ctx.mark_open(&node);
            }
        }
        ctx.mark_closed(&q);
    }
    let search_time = (time::precise_time_ns() -
                       ctx.start_time) as f64 / 1000000.0;
    println!("No path found for {} ({:?} to {:?} (distance: {}, tested: {}, {} ms)!",
             fig.get_name(), start_pos, end_pos,
             est_pos_distance(start_pos, end_pos),
             ctx.node_by_id.len(), search_time);
    return vec![];
}
