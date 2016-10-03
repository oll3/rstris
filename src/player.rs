extern crate rand;

use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use std::collections::BinaryHeap;

use rstris::playfield::*;
use rstris::figure::*;
use rstris::figure_pos::*;
use rstris::position::*;
use rstris::block::*;


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

    fn new_figure_event(&mut self, _: u64,
                        _: &Playfield, _: &FigurePos);

    fn figure_move_event(&mut self, pf: &Playfield,
                         movement: Movement, time: u64);


    fn handle_move(&mut self, pf: &mut Playfield,
                   movement: MoveAndTime) {
        self.common_mut().handle_move(pf, movement);
    }

    fn try_place_new_figure(&mut self, ticks: u64,
                            pf: &mut Playfield) -> BlockState {
        let result = self.common_mut().try_place_new_figure(ticks, pf);
        if result == BlockState::NotSet {
            let fig = &self.common().figure_in_play.clone().unwrap();
            self.new_figure_event(ticks, pf, &fig);
        }
        return result;
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
        self.set_time_of_last_move(&movement, ticks);
        let move_time = MoveAndTime{movement: movement, time: ticks};
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

    fn update(&mut self, ticks: u64, pf: &Playfield) {
        if self.figure_in_play() {
            let time_since_down =
                self.time_since_move(ticks, &Movement::MoveDown);
            if time_since_down >= self.force_down_time as i64 {
                let overdue = time_since_down - self.force_down_time as i64;
                println!("Move down (ticks: {}, overdue: {})",
                         ticks, overdue);
                self.add_move(Movement::MoveDown,
                              (ticks as i64) as u64);
            }
        }
    }

    //
    // Move player current figure according to the given movements.
    // If movement caused full lines being created then return those
    // line indexes.
    //
    fn handle_move(&mut self, pf: &mut Playfield,
                   move_and_time: MoveAndTime) {

        let (fig_move, move_time) = (move_and_time.movement,
                                     move_and_time.time);
        let mut fig_pos = self.get_figure().unwrap();
        fig_pos.remove(pf);
        let test_pos = PosDir::apply_move(fig_pos.get_position(), &fig_move);

        let collision = fig_pos.get_figure().test_collision(pf, &test_pos);
        if collision == BlockState::Locked && fig_move == Movement::MoveDown {
            fig_pos.lock(pf);
            self.set_figure(None);
        }
        else {
            if collision == BlockState::NotSet {
                fig_pos.set_position(&test_pos);
            }
            fig_pos.place(pf);
            self.set_figure(Some(fig_pos));
        }
    }

    fn try_place_new_figure(&mut self, ticks: u64,
                            pf: &mut Playfield) -> BlockState {

        let figure = self.next_figure().clone();
        let pos = PosDir::new((pf.width() / 2 - 1) as i32, 0, 0);
        if figure.collide_locked(pf, &pos) {
            println!("Figure collided with locked block");
            return BlockState::Locked;
        } else if figure.collide_any(pf, &pos) {
            println!("Figure collided with blocking block");
            return BlockState::InFlight;
        }
        let fig_pos = FigurePos::new(figure, pos);
        self.gen_next_figure();
        let next_down = ticks + self.force_down_time;
        self.add_move(Movement::MoveDown, next_down);

        println!("{}: Placed figure {} in playfield (next is {})",
                 self.get_name(), fig_pos.get_figure().get_name(),
                 self.next_figure().get_name());
        fig_pos.place(pf);
        self.set_figure(Some(fig_pos));
        return BlockState::NotSet;
    }
}
