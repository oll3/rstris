extern crate sdl2;
extern crate time;
extern crate rstris;
extern crate rustc_serialize;

mod draw;
mod player;

use std::io;
use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json;

use player::*;
use draw::*;
use rstris::find::*;
use rstris::block::*;
use rstris::playfield::*;
use rstris::figure::*;
use rstris::figure_pos::*;
use rstris::position::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;


static PF_WIDTH: u32 = 10;
static PF_HEIGHT: u32 = 20;
static BLOCK_SIZE: u32 = 20;
static BLOCK_SPACING: u32 = 1;
static FRAME_COLOR: Color = Color::RGB(200, 64, 64);
static FILL_COLOR: Color = Color::RGB(98, 204, 244);
static BG_COLOR: Color = Color::RGB(101, 208, 246);
static DELAY_FIRST_STEP_DOWN: u64 = 1 * 1000 * 1000 * 1000;

struct KeyMap {
    step_left: Option<Keycode>,
    step_right: Option<Keycode>,
    step_down: Option<Keycode>,
    rot_cw: Option<Keycode>,
    rot_ccw: Option<Keycode>,
}

struct HumanPlayer {
    player: PlayerData,
    key_map: KeyMap,
}

impl Player for HumanPlayer {
    fn get_player_data(&self) -> &PlayerData {
        &self.player
    }
    fn get_player_data_mut(&mut self) -> &mut PlayerData {
        &mut self.player
    }
    fn handle_input(&mut self, current_ticks: u64,
                    pressed_keys: &mut HashMap<Keycode, u64>)
                    -> Vec<(Movement, u64)> {
        let mut moves: Vec<(Movement, u64)> = vec![];
        let keys = pressed_keys.clone();
        for (key, pressed_at) in keys {
            match key_to_movement(&self.key_map, key) {
                Some(movement) => {
                    let time_last_move = current_ticks -
                        match self.player.time_last_move.get(&movement) {
                            Some(t) => *t,
                            None => 0
                        };
                    let time_pressed = current_ticks - pressed_at;
                    if current_ticks <= pressed_at {
                        moves.push((movement, current_ticks));
                    } else if time_pressed > 200000000 &&
                        time_last_move > 50000000
                    {
                        moves.push((movement, current_ticks));
                    }
                }
                None => {}
            }
        }
        return moves;
    }
}

impl HumanPlayer {
    pub fn new(player: PlayerData, key_map: KeyMap) -> Self {
        HumanPlayer {
            player: player,
            key_map: key_map,
        }
    }
}


struct PlayfieldContext<'a> {
    pf: Playfield,
    players: Vec<&'a mut Player>,
    game_over: bool,
    lines_to_throw: Vec<usize>,
}


impl <'a>PlayfieldContext<'a> {
    pub fn new(pf: Playfield) -> Self {
        PlayfieldContext{pf: pf,
                         players: Vec::new(),
                         game_over: false,
                         lines_to_throw: Vec::new()}
    }

    pub fn add_player(&mut self, player: &'a mut Player) {
        self.players.push(player);
    }

    // Test if there are figures currently being played
    pub fn figures_in_play(&self) -> bool {
        for player in &self.players {
            if player.get_player_data().figure_is_in_play() {
                return true;
            }
        }
        return false;
    }
}

//
// Build list of figures
//
fn init_figures() -> Vec<Figure> {
    let mut figure_list: Vec<Figure> = Vec::new();
    figure_list.push(Figure::
                     new_from_face("1",
                                   &[&[0, 0, 0],
                                     &[1, 1, 1],
                                     &[0, 1, 0]]));
    figure_list.push(Figure::
                     new_from_face("2",
                                   &[&[0, 0, 0],
                                     &[2, 2, 2],
                                     &[0, 0, 2]]));
    figure_list.push(Figure::
                     new_from_face("3",
                                   &[&[0, 0, 3],
                                     &[3, 3, 3],
                                     &[0, 0, 0]]));
    figure_list.push(Figure::
                     new_from_face("4",
                                   &[&[4, 4],
                                     &[4, 4]]));
    figure_list.push(Figure::
                     new_from_face("5",
                                   &[&[0, 5, 5],
                                     &[5, 5, 0]]));
    figure_list.push(Figure::
                     new_from_face("6",
                                   &[&[6, 6, 0],
                                     &[0, 6, 6]]));
    figure_list.push(Figure::
                     new_from_face("7",
                                   &[&[0, 7, 0],
                                     &[0, 7, 0],
                                     &[0, 7, 0],
                                     &[0, 7, 0]]));
    return figure_list;
}

fn get_max_figure_dimensions(figure_list: &Vec<Figure>)
                             -> (u32, u32) {
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;
    for fig in figure_list {
        for dir in fig.dir.clone() {
            if dir.get_width() as u32 > max_width {
                max_width = dir.get_width() as u32;
            }
            if dir.get_height() as u32 > max_height {
                max_height = dir.get_height() as u32;
            }
        }
    }
    return (max_width, max_height);
}

fn pf_to_file(pf: &Playfield, file_name: String) -> Result<(), io::Error> {
    let mut buffer = try!(File::create(file_name));
    match json::encode(&pf) {
        Ok(j) => {
            try!(buffer.write_all(j.as_bytes()));
        },
        Err(_) => {},
    }
    return Ok(());
}

//
// Try to place a new figure in playfield
//
fn place_new_figure(player: &mut Player,
                    pf: &mut Playfield) -> bool {

    let current_ticks = time::precise_time_ns();
    // Place new figure in playfield
    let player_data = player.get_player_data_mut();
    let figure = player_data.get_next_figure().clone();
    let pos = Position::new((pf.width() / 2 - 1) as i32, 0, 0);
    if figure.collide_locked(pf, &pos) {
        println!("{}: Game over!", player_data.get_name());
        return false;
    }
    if figure.collide_blocked(pf, &pos) {
        return true;
    }
    player_data.gen_next_figure();
    println!("{}: Placed figure {} in playfield (next is {})",
             player_data.get_name(), figure.get_name(),
             player_data.get_next_figure().get_name());
    let fig_pos = FigurePos::new(figure, pos);
    fig_pos.place(pf);
    player_data.set_figure(Some(fig_pos));
    player_data.delay_first_step_down =
        current_ticks + DELAY_FIRST_STEP_DOWN;
    return true;
}

//
// Move player current figure according to the given movements.
// If movement caused full lines being created then return those
// line indexes.
//
/*
fn handle_player_moves(player: &mut Player, pf: &mut Playfield,
                       moves: Vec<Movement>) -> Vec<usize> {

    let mut lock_figure = false;
    let current_ticks = time::precise_time_ns();
    let player_data = player.get_player_data_mut();
    let mut fig_pos = player_data.get_figure().unwrap();
    let mut new_pos = fig_pos.get_position().clone();
    fig_pos.remove(pf);

    for fig_move in moves {
        player_data.set_time_of_move(fig_move.clone(), current_ticks);
        player_data.delay_first_step_down = 0;
        let fig = fig_pos.get_figure();
        let test_pos = Position::apply_move(fig_pos.get_position(), &fig_move);
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
                 player_data.get_name(), lines_to_test);
        let locked_lines = pf.get_locked_lines(&lines_to_test);
        println!("{}: Found locked lines at: {:?}",
                 player_data.get_name(), locked_lines);
        player_data.stats.line_count += locked_lines.len();
        player_data.set_figure(None);
        return locked_lines;
    } else {
        fig_pos.place(pf);
        player_data.set_figure(Some(fig_pos));
    }
    return vec![];
}*/

fn key_to_movement(key_map: &KeyMap, key: Keycode) -> Option<Movement> {
    if !key_map.step_left.is_none() &&
        key == key_map.step_left.unwrap()
    {
        return Some(Movement::MoveLeft);
    } else if !key_map.step_right.is_none() &&
        key == key_map.step_right.unwrap()
    {
        return Some(Movement::MoveRight);
    } else if !key_map.step_down.is_none() &&
        key == key_map.step_down.unwrap()
    {
        return Some(Movement::MoveDown);
    } else if !key_map.rot_cw.is_none() &&
        key == key_map.rot_cw.unwrap()
    {
        return Some(Movement::RotateCW);
    }
    else if !key_map.rot_ccw.is_none() &&
        key == key_map.rot_ccw.unwrap()
    {
        return Some(Movement::RotateCCW);
    }
    return None;
}


fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let figure_list = init_figures();
    let (figure_max_width, figure_max_height) =
        get_max_figure_dimensions(&figure_list);
    println!("Max figure dimensions: {} x {}",
             figure_max_width, figure_max_height);

    let window_width: u32 = (PF_WIDTH + 2 + figure_max_width + 3) *
        (BLOCK_SIZE + BLOCK_SPACING);
    let window_height: u32 = (PF_HEIGHT + 1) * (BLOCK_SIZE + BLOCK_SPACING);
    let window = video_subsystem.window("rust-sdl2 demo: Video",
                                        window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut renderer = window.renderer().build().unwrap();
    let mut draw = DrawContext::new(BLOCK_SIZE,
                                    BLOCK_SPACING,
                                    FRAME_COLOR,
                                    FILL_COLOR);

    let player1_key_map = KeyMap {
        step_left: Some(Keycode::Left),
        step_right: Some(Keycode::Right),
        step_down: Some(Keycode::Down),
        rot_cw: Some(Keycode::Up),
        rot_ccw: None
    };

    let player2_key_map = KeyMap {
        step_left: Some(Keycode::A),
        step_right: Some(Keycode::D),
        step_down: Some(Keycode::S),
        rot_cw: Some(Keycode::W),
        rot_ccw: None
    };
    let mut player1 = HumanPlayer::new(
        PlayerData::new("Human 1", figure_list.clone()),
        player1_key_map
    );
    let mut player2 = HumanPlayer::new(
        PlayerData::new("Human 2", figure_list.clone()),
        player2_key_map
    );

    let pf1 = Playfield::new("Playfield 1",
                             PF_WIDTH as usize, PF_HEIGHT as usize);
    let mut pf_ctx = PlayfieldContext::new(pf1);


    pf_ctx.add_player(&mut player1);
    pf_ctx.add_player(&mut player2);

    let mut pause = false;
    let mut frame_cnt_sec = 0;
    let mut sec_timer = time::precise_time_ns();
    let mut pressed_keys: HashMap<Keycode, u64> = HashMap::new();
    let mut events = sdl_context.event_pump().unwrap();
    'running: loop {
        let current_ticks = time::precise_time_ns();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {
                    keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    pause = !pause;
                    if pause {
                        println!("Paused");
                    } else {
                        println!("Continued");
                    }
                },Event::KeyDown {
                    keycode: Some(key), .. } => {
                    if !pressed_keys.contains_key(&key) {
                        pressed_keys.insert(key, current_ticks);
                    }
                },Event::KeyUp {
                    keycode: Some(key), .. } => {
                    if pressed_keys.contains_key(&key) {
                        pressed_keys.remove(&key);
                    }
                },

                _ => {}
            }
        }

        if pause || pf_ctx.game_over {
            continue;
        }


        // Handle movement and figure creation
        for player in & mut pf_ctx.players {
            let mut moves: Vec<(Movement, u64)> = Vec::new();

            moves.append(&mut player.handle_input(current_ticks,
                                                  &mut pressed_keys));
            moves.append(&mut player.get_player_data_mut().move_every(
                current_ticks,
                Movement::MoveDown,
                500000000 /* ns */
            )
            );

            if player.get_player_data_mut().figure_in_play.is_some() {
                // Player has a figure in game
                if moves.len() > 0 {
                    let mut lines =
                        player.get_player_data_mut().
                        handle_moves(&mut pf_ctx.pf, moves);
                    pf_ctx.pf.set_lines(&lines, &Block::new_locked(10));
                    pf_ctx.lines_to_throw.append(&mut lines);
                }
            } else if pf_ctx.lines_to_throw.len() == 0 {
                if !place_new_figure(*player, &mut pf_ctx.pf) {
                    pf_ctx.game_over = true;
                }
            }
        }

        // Throw full lines (when there are no figures being played)
        if pf_ctx.lines_to_throw.len() > 0 && !pf_ctx.figures_in_play() {
            pf_ctx.lines_to_throw.sort();
            pf_ctx.lines_to_throw.dedup();
            println!("Throw away lines: {:?}", pf_ctx.lines_to_throw);
            for line in &pf_ctx.lines_to_throw {
                pf_ctx.pf.throw_line(*line);
            }
            pf_ctx.lines_to_throw.clear();
        }

        /* Render graphics */
        draw.clear(&mut renderer, BG_COLOR);
        draw.draw_playfield(&mut renderer, &pf_ctx.pf);
        let mut pi = 0;
        for player in &mut pf_ctx.players {
            let player_data = player.get_player_data();
            draw.draw_next_figure(&mut renderer,
                                  &player_data.get_next_figure(),
                                  PF_WIDTH + 3,
                                  (figure_max_height + 1) * pi,
                                  figure_max_width, figure_max_height);

            pi += 1;
        }
        draw.present(&mut renderer);

        /* Write FPS in window title */
        frame_cnt_sec += 1;
        if (current_ticks - sec_timer) >= 1000000000 {
            let title = format!("RSTris (fps: {})", frame_cnt_sec);
            let mut window = renderer.window_mut().unwrap();

            frame_cnt_sec = 0;
            sec_timer = current_ticks;
            window.set_title(&title).unwrap();
        }
        std::thread::sleep(std::time::Duration::new(0, 10000000));
    }
}
