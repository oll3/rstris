extern crate rand;

use std::collections::HashMap;
use sdl2::keyboard::Keycode;

use rstris::playfield::*;
use rstris::figure::*;
use rstris::figure_pos::*;
use rstris::position::*;

static DELAY_FIRST_STEP_DOWN: u64 = 1 * 1000 * 1000 * 1000;

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
    delay_first_step_down: u64,
}

pub trait Player {
    fn common(&self) -> &PlayerCommon;
    fn common_mut(&mut self) -> &mut PlayerCommon;
    fn handle_input(&mut self, current_ticks: u64,
                    pressed_keys: &mut HashMap<Keycode, u64>)
                    -> Vec<(Movement, u64)>;
}

impl PlayerCommon {
    pub fn new(name: &str, figures: Vec<Figure>) -> Self {
        PlayerCommon {
            name: name.to_owned(),
            stats: PlayerStats {
                line_count: 0,
            },
            time_last_move: HashMap::new(),
            next_figure: PlayerCommon::get_rand_figure(&figures).clone(),
            avail_figures: figures,
            delay_first_step_down: 0,
            figure_in_play: None,
        }
    }
    fn get_rand_figure(figures: &Vec<Figure>) -> &Figure {
        let next_figure = (rand::random::<u8>() %
                           figures.len() as u8) as usize;
        return &figures[next_figure];
    }
    pub fn get_next_figure(&self) -> &Figure {
        &self.next_figure
    }
    pub fn gen_next_figure(&mut self) {
        self.next_figure =
            PlayerCommon::get_rand_figure(&self.avail_figures).clone();
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
    pub fn figure_is_in_play(&self) -> bool {
        self.figure_in_play.is_some()
    }
    pub fn set_time_of_move(&mut self, fig_move: Movement, time: u64) {
        self.time_last_move.insert(fig_move, time);
    }

    pub fn figure_in_play(&self) -> bool {
        self.figure_in_play.is_some()
    }

    //
    // Move player current figure according to the given movements.
    // If movement caused full lines being created then return those
    // line indexes.
    //
    pub fn handle_moves(&mut self, pf: &mut Playfield,
                        moves: Vec<(Movement, u64)>) -> Vec<usize> {

        let mut lock_figure = false;
        let mut fig_pos = self.get_figure().unwrap();
        let mut new_pos = fig_pos.get_position().clone();
        fig_pos.remove(pf);

        for (fig_move, move_time) in moves {
            self.set_time_of_move(fig_move.clone(), move_time);
            self.delay_first_step_down = 0;
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

    pub fn place_new_figure(&mut self, current_ticks: u64,
                            pf: &mut Playfield) -> bool {

        // Place new figure in playfield
        let figure = self.get_next_figure().clone();
        let pos = Position::new((pf.width() / 2 - 1) as i32, 0, 0);
        if figure.collide_locked(pf, &pos) {
            println!("{}: Game over!", self.get_name());
            return false;
        }
        if figure.collide_blocked(pf, &pos) {
            return true;
        }
        self.gen_next_figure();
        println!("{}: Placed figure {} in playfield (next is {})",
                 self.get_name(), figure.get_name(),
                 self.get_next_figure().get_name());
        let fig_pos = FigurePos::new(figure, pos);
        fig_pos.place(pf);
        self.set_figure(Some(fig_pos));
        self.delay_first_step_down =
            current_ticks + DELAY_FIRST_STEP_DOWN;
        return true;
    }


    pub fn move_every(&self, current_ticks: u64,
                      movement: Movement,
                      every_ns: u64) -> Vec<(Movement, u64)> {
        let mut moves: Vec<(Movement, u64)> = vec![];
        if current_ticks > self.delay_first_step_down {
            let last_move = self.time_last_move.get(&movement);
            if last_move.is_none() ||
                (last_move.unwrap() + every_ns) < current_ticks
            {
                moves.push((movement, current_ticks));
            }
        }
        return moves;
    }
}
