extern crate rand;

use std::collections::HashMap;
use sdl2::keyboard::Keycode;

use rstris::playfield::*;
use rstris::figure::*;
use rstris::figure_pos::*;
use rstris::position::*;


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
}

pub trait Player {

    fn common(&self) -> &PlayerCommon;
    fn common_mut(&mut self) -> &mut PlayerCommon;
    fn get_moves(&mut self, current_ticks: u64) ->  Vec<(Movement, u64)>;
    fn update(&mut self, _: u64, _: &Playfield);

    fn handle_new_figure(&mut self, current_ticks: u64,
                         pf: &Playfield, fig_pos: &FigurePos) {
        // Implement if needed
    }

    fn handle_input(&mut self, _: u64, _: &mut HashMap<Keycode, u64>) {
        // Implement if needed
    }

    fn handle_moves(&mut self, pf: &mut Playfield,
                    moves: Vec<(Movement, u64)>) -> Vec<usize> {
        self.common_mut().handle_moves(pf, moves)
    }

    fn place_new_figure(&mut self, current_ticks: u64,
                        pf: &mut Playfield) -> bool {

        // Place new figure in playfield
        let figure = self.common().next_figure().clone();
        let pos = Position::new((pf.width() / 2 - 1) as i32, 0, 0);
        if figure.collide_locked(pf, &pos) {
            println!("Figure collided with locked block");
            return false;
        } else if figure.collide_blocked(pf, &pos) {
            println!("Figure collided with blocking block");
            return true;
        }
        let fig_pos = FigurePos::new(figure, pos);
        self.common_mut().gen_next_figure();
        self.handle_new_figure(current_ticks, pf, &fig_pos);
        return self.common_mut().place_new_figure(current_ticks, pf, fig_pos);
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

    fn set_time_of_move(&mut self, fig_move: Movement, time: u64) {
        self.time_last_move.insert(fig_move, time);
    }

    fn figure_in_play(&self) -> bool {
        self.figure_in_play.is_some()
    }

    //
    // Move player current figure according to the given movements.
    // If movement caused full lines being created then return those
    // line indexes.
    //
    fn handle_moves(&mut self, pf: &mut Playfield,
                        moves: Vec<(Movement, u64)>) -> Vec<usize> {

        let mut lock_figure = false;
        let mut fig_pos = self.get_figure().unwrap();
        let mut new_pos = fig_pos.get_position().clone();
        fig_pos.remove(pf);

        for (fig_move, move_time) in moves {
            self.set_time_of_move(fig_move.clone(), move_time);
            let fig = fig_pos.get_figure();
            let test_pos =
                Position::apply_move(fig_pos.get_position(), &fig_move);
            let test_pos_locked = fig.collide_locked(pf, &test_pos);
            let test_pos_blocked = fig.collide_blocked(pf, &test_pos);
            if !test_pos_locked && !test_pos_blocked {
                new_pos = test_pos;
            } else if fig_move == Movement::MoveDown && test_pos_locked {
                // Figure couldn't be moved down further because of collision
                // with locked block(s) - Mark figure blocks as locked in its
                // current position.
                lock_figure = true;
                break;
            } else {
                // Move is not valid so the rest of the
                // moves are not valid either.
                break;
            }
        }
        fig_pos.set_position(&new_pos);

        if lock_figure {
            fig_pos.lock(pf);
            let fig_dir = fig_pos.get_figure_dir();
            let mut lines_to_test: Vec<usize> = Vec::new();
            for l in fig_dir.get_row_with_blocks() {
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

    fn place_new_figure(&mut self, current_ticks: u64,
                        pf: &mut Playfield, fig_pos: FigurePos) -> bool {

        println!("{}: Placed figure {} in playfield (next is {})",
                 self.get_name(), fig_pos.get_figure().get_name(),
                 self.next_figure().get_name());
        fig_pos.place(pf);
        self.set_figure(Some(fig_pos));
        return true;
    }
}
