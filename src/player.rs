extern crate rand;

use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use rstris::playfield::*;
use rstris::figure::*;
use rstris::figure_pos::*;
use rstris::position::*;


pub struct PlayerStats {
    pub line_count: usize,
}

#[derive(Debug)]
struct MoveTime {
    movement: Movement,
    time: u64,
}
impl Ord for MoveTime {
    fn cmp(&self, other: &MoveTime) -> Ordering {
        other.time.cmp(&self.time)
    }
}
impl PartialOrd for MoveTime {
    fn partial_cmp(&self, other: &MoveTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for MoveTime {}
impl PartialEq for MoveTime {
    fn eq(&self, other: &MoveTime) -> bool { self.time == other.time }
}


pub struct PlayerCommon {
    name: String,
    pub time_last_move: HashMap<Movement, u64>,
    avail_figures: Vec<Figure>,
    next_figure: Figure,
    figure_in_play: Option<FigurePos>,
    pub stats: PlayerStats,
    pub force_down_time: u64,
    move_queue: BinaryHeap<MoveTime>,
}

pub trait Player {

    fn common(&self) -> &PlayerCommon;
    fn common_mut(&mut self) -> &mut PlayerCommon;
    fn update(&mut self, _: u64, _: &Playfield);

    fn handle_new_figure(&mut self, ticks: u64,
                         pf: &Playfield, fig_pos: &FigurePos) {
        // Implement if needed
    }

    fn handle_input(&mut self, _: u64, _: &mut HashMap<Keycode, u64>) {
        // Implement if needed
    }

    fn handle_move(&mut self, pf: &mut Playfield,
                   movement: MoveTime) -> Vec<usize> {
        self.common_mut().handle_move(pf, movement)
    }

    fn place_new_figure(&mut self, ticks: u64,
                        pf: &mut Playfield) -> bool {

        // Place new figure in playfield
        let figure = self.common().next_figure().clone();
        let pos = PosDir::new((pf.width() / 2 - 1) as i32, 0, 0);
        if figure.collide_locked(pf, &pos) {
            println!("Figure collided with locked block");
            return false;
        } else if figure.collide_blocked(pf, &pos) {
            println!("Figure collided with blocking block");
            return true;
        }
        let fig_pos = FigurePos::new(figure, pos);
        self.common_mut().gen_next_figure();
        self.common_mut().move_queue.clear();
        self.handle_new_figure(ticks, pf, &fig_pos);
        return self.common_mut().place_new_figure(ticks, pf, fig_pos);
    }

    fn next_figure(&self) -> &Figure {
        self.common().next_figure()
    }

    fn figure_in_play(&self) -> bool {
        self.common().figure_in_play()
    }
}

impl PlayerCommon {

    pub fn new(name: &str, force_down_time: u64,
               figures: Vec<Figure>) -> Self {
        PlayerCommon {
            name: name.to_owned(),
            stats: PlayerStats {
                line_count: 0,
            },
            time_last_move: HashMap::new(),
            next_figure: PlayerCommon::get_rand_figure(&figures).clone(),
            avail_figures: figures,
            figure_in_play: None,
            force_down_time: force_down_time,
            move_queue: BinaryHeap::new(),
        }
    }

    fn get_rand_figure(figures: &Vec<Figure>) -> &Figure {
        let next_figure = (rand::random::<u8>() %
                           figures.len() as u8) as usize;
        return &figures[next_figure];
    }

    fn next_figure(&self) -> &Figure {
        &self.next_figure
    }

    fn gen_next_figure(&mut self) {
        self.next_figure =
            PlayerCommon::get_rand_figure(&self.avail_figures).clone();
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    fn set_figure(&mut self, figure: Option<FigurePos>) {
        self.figure_in_play = figure;
    }

    pub fn get_figure(&self) -> Option<FigurePos> {
        self.figure_in_play.clone()
    }

    pub fn add_move(&mut self, movement: Movement, ticks: u64) {
        let move_time = MoveTime{movement: movement, time: ticks};
        println!("Add move {:?}", move_time);
        self.move_queue.push(move_time);
    }

    fn time_for_next_move(&self, ticks: u64) -> bool {
        if let Some(move_and_time) = self.move_queue.peek() {
            if move_and_time.time <= ticks {
                return true;
            }
        }
        return false;
    }

    pub fn get_next_move(&mut self, ticks: u64) -> Option<MoveTime> {
        if self.time_for_next_move(ticks) {
            return Some(self.move_queue.pop().unwrap());
        }
        return None;
    }

    fn set_time_of_move(&mut self, fig_move: Movement, time: u64) {
        self.time_last_move.insert(fig_move, time);
    }

    pub fn time_of_last_move(&self, movement: Movement) -> u64 {
        if let Some(time) = self.time_last_move.get(&movement) {
            return *time;
        }
        return 0;
    }

    pub fn time_until_down(&self, ticks: u64) -> i64 {
        self.time_of_last_move(Movement::MoveDown) as i64 +
            self.force_down_time as i64 - ticks as i64
    }

    fn figure_in_play(&self) -> bool {
        self.figure_in_play.is_some()
    }

    //
    // Move player current figure according to the given movements.
    // If movement caused full lines being created then return those
    // line indexes.
    //
    fn handle_move(&mut self, pf: &mut Playfield,
                   movement: MoveTime) -> Vec<usize> {
        let (fig_move, move_time) = (movement.movement, movement.time);
        let mut lock_figure = false;
        let mut fig_pos = self.get_figure().unwrap();
        let mut new_pos = fig_pos.get_position().clone();
        fig_pos.remove(pf);

        self.set_time_of_move(fig_move.clone(), move_time);
        {
            let fig = fig_pos.get_figure();
            let test_pos =
                PosDir::apply_move(fig_pos.get_position(), &fig_move);
            let test_pos_locked = fig.collide_locked(pf, &test_pos);
            let test_pos_blocked = fig.collide_blocked(pf, &test_pos);
            if !test_pos_locked && !test_pos_blocked {
                new_pos = test_pos;
            } else if fig_move == Movement::MoveDown && test_pos_locked {
                // Figure couldn't be moved down further because of collision
                // with locked block(s) - Mark figure blocks as locked in its
                // current position.
                lock_figure = true;
            } else {
                // Move is not valid so the rest of the
                // moves are not valid either.
            }
        }
        fig_pos.set_position(&new_pos);

        if lock_figure {
            fig_pos.lock(pf);
            let face = fig_pos.get_face();
            let mut lines_to_test: Vec<usize> = Vec::new();
            for l in face.get_row_with_blocks() {
                lines_to_test.push(l + fig_pos.get_position().get_y() as usize);
            }
            println!("{}: Test for locked lines at: {:?}...",
                     self.get_name(), lines_to_test);
            let locked_lines = pf.get_locked_lines(&lines_to_test);
            println!("{}: Found locked lines at: {:?}",
                     self.get_name(), locked_lines);
            self.stats.line_count += locked_lines.len();
            self.set_figure(None);
            return locked_lines;
        } else {
            fig_pos.place(pf);
            self.set_figure(Some(fig_pos));
        }
        return vec![];
    }

    fn place_new_figure(&mut self, ticks: u64,
                        pf: &mut Playfield, fig_pos: FigurePos) -> bool {

        println!("{}: Placed figure {} in playfield (next is {})",
                 self.get_name(), fig_pos.get_figure().get_name(),
                 self.next_figure().get_name());
        fig_pos.place(pf);
        self.set_figure(Some(fig_pos));
        return true;
    }
}
