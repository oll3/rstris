extern crate rand;

use sdl2::keyboard::Keycode;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use rstris::figure::*;
use rstris::figure_pos::*;
use rstris::movement::*;
use rstris::playfield::*;

pub struct PlayerStats {
    pub line_count: usize,
}

pub struct PlayerCommon {
    name: String,
    pub time_last_move: HashMap<Movement, u64>,
    avail_figures: Vec<Figure>,
    next_figure: Figure,
    figure_in_play: Option<FigurePos>,
    pub stats: PlayerStats,
    pub force_down_time: u64,
    move_queue: BinaryHeap<MoveAndTime>,
}

pub trait Player {
    fn common(&self) -> &PlayerCommon;
    fn common_mut(&mut self) -> &mut PlayerCommon;
    fn update(&mut self, ticks: u64, pf: &Playfield) {
        self.common_mut().update(ticks, pf);
    }

    fn handle_input(&mut self, _: u64, _: &mut HashMap<Keycode, u64>) {
        // Implement if needed
    }

    fn new_figure_event(&mut self, _: u64, _: &Playfield, _: &FigurePos);

    fn figure_move_event(
        &mut self,
        ticks: u64,
        pf: &Playfield,
        fig_pos: &FigurePos,
        movement: &Movement,
    );

    fn next_figure(&self) -> &Figure {
        self.common().next_figure()
    }

    fn figure_in_play(&self) -> bool {
        self.common().figure_in_play()
    }
}

impl PlayerCommon {
    pub fn new(name: &str, force_down_time: u64, figures: Vec<Figure>) -> Self {
        PlayerCommon {
            name: name.to_owned(),
            stats: PlayerStats { line_count: 0 },
            time_last_move: HashMap::new(),
            next_figure: PlayerCommon::get_rand_figure(&figures).clone(),
            avail_figures: figures,
            figure_in_play: None,
            force_down_time: force_down_time,
            move_queue: BinaryHeap::new(),
        }
    }

    fn get_rand_figure(figures: &Vec<Figure>) -> &Figure {
        let next_figure = (rand::random::<u8>() % figures.len() as u8) as usize;
        return &figures[next_figure];
    }

    pub fn next_figure(&self) -> &Figure {
        &self.next_figure
    }

    pub fn gen_next_figure(&mut self) {
        self.next_figure = PlayerCommon::get_rand_figure(&self.avail_figures).clone();
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_figure(&mut self, figure: Option<FigurePos>) {
        self.figure_in_play = figure;
    }

    pub fn get_figure(&self) -> Option<FigurePos> {
        self.figure_in_play.clone()
    }

    pub fn add_move(&mut self, movement: Movement, ticks: u64) {
        self.set_time_of_last_move(&movement, ticks);
        let move_time = MoveAndTime {
            movement: movement,
            time: ticks,
        };
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

    pub fn get_next_move(&mut self, ticks: u64) -> Option<MoveAndTime> {
        if self.time_for_next_move(ticks) {
            return Some(self.move_queue.pop().unwrap());
        }
        return None;
    }

    fn set_time_of_last_move(&mut self, movement: &Movement, time: u64) {
        if time > self.time_last_move(movement) {
            self.time_last_move.insert(movement.clone(), time);
        }
    }

    pub fn time_last_move(&self, movement: &Movement) -> u64 {
        if let Some(time) = self.time_last_move.get(movement) {
            return *time;
        }
        return 0;
    }

    pub fn time_since_move(&self, ticks: u64, movement: &Movement) -> i64 {
        let last_move = self.time_last_move(movement) as i64;
        ticks as i64 - last_move
    }

    fn figure_in_play(&self) -> bool {
        self.figure_in_play.is_some()
    }

    fn update(&mut self, ticks: u64, _: &Playfield) {
        if self.figure_in_play() {
            let time_since_down = self.time_since_move(ticks, &Movement::MoveDown);
            if time_since_down >= self.force_down_time as i64 {
                self.add_move(Movement::MoveDown, (ticks as i64) as u64);
            }
        }
    }
}
