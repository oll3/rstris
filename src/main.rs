extern crate rand;
extern crate sdl2;
extern crate time;
extern crate rstris;
extern crate rustc_serialize;

mod draw;
mod player;
mod human_player;
mod computer_player;

use std::io;
use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json;

use player::*;
use human_player::*;
use computer_player::*;
use draw::*;
use rstris::block::*;
use rstris::playfield::*;
use rstris::figure::*;
use rstris::position::*;
use rstris::figure_pos::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;


static PF_WIDTH: u32 = 16;
static PF_HEIGHT: u32 = 30;
static BLOCK_SIZE: u32 = 20;
static BLOCK_SPACING: u32 = 1;
static FRAME_COLOR: Color = Color::RGB(200, 64, 64);
static FILL_COLOR: Color = Color::RGB(98, 204, 244);
static BG_COLOR: Color = Color::RGB(101, 208, 246);

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
            if player.figure_in_play() {
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
        for face in fig.faces() {
            if face.get_width() as u32 > max_width {
                max_width = face.get_width() as u32;
            }
            if face.get_height() as u32 > max_height {
                max_height = face.get_height() as u32;
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

struct RandomComputer {}
impl ComputerType for RandomComputer {
    fn eval_placing(&mut self, _: &FigurePos, _: &Playfield) -> i32 {
        rand::random::<i32>()
    }
}

fn lowest_block(fig_pos: &FigurePos) -> i32 {
    fig_pos.get_face().get_lowest_block().get_y() +
        fig_pos.get_position().get_y()
}

struct LowestPossibleComputer {}
impl ComputerType for LowestPossibleComputer {
    fn eval_placing(&mut self, fig_pos: &FigurePos, _: &Playfield) -> i32 {
        lowest_block(fig_pos)
    }
}

struct SmarterComputer {
    num_voids_pre: u32,
}
impl SmarterComputer {
    fn new() -> Self {
        SmarterComputer{num_voids_pre: 0}
    }
}

impl ComputerType for SmarterComputer {
    fn init_eval(&mut self, pf: &Playfield, _: usize) {
        self.num_voids_pre = pf.count_voids();
        println!("Num voids: {}", self.num_voids_pre);
    }
    fn eval_placing(&mut self, fig_pos: &FigurePos, pf: &Playfield) -> i32 {
        let bottom_block = lowest_block(fig_pos);
        let mut pf = pf.clone();
        fig_pos.lock(&mut pf);

        let mut blocks_below_score = 0;
        let fig_face = fig_pos.get_face();
        for block_pos in fig_face.get_block_positions() {
            let test_pos =
                block_pos + fig_pos.get_position().get_pos().clone() +
                Position::new(0, 1);
            if pf.contains(&test_pos) && pf.block_is_set(&test_pos) {
                blocks_below_score += 1;
            }
        }

        let new_voids = pf.count_voids() as i32 - self.num_voids_pre as i32;
//        println!("New voids: {}", new_voids);

        return bottom_block - new_voids * 2 + blocks_below_score;
    }
}


struct JitterComputer {
    pre_col_jitter: i32,
    pre_row_jitter: i32,
    pre_voids: i32,
}
impl JitterComputer {
    fn new() -> Self {
        JitterComputer{
            pre_col_jitter: 0,
            pre_row_jitter: 0,
            pre_voids: 0,
        }
    }
}
impl ComputerType for JitterComputer {
    fn init_eval(&mut self, pf: &Playfield, _: usize) {
        self.pre_voids = pf.count_voids() as i32;
        self.pre_col_jitter = pf.get_col_jitter() as i32;
        self.pre_row_jitter = pf.get_row_jitter() as i32;
    }

    fn eval_placing(&mut self, fig_pos: &FigurePos, pf: &Playfield) -> i32 {
        let mut pf = pf.clone();
        fig_pos.lock(&mut pf);
        let bottom_block = lowest_block(fig_pos);
        // Measure playfield jitter - The lower jitter the better
        let col_jitter = pf.get_col_jitter() as i32 - self.pre_col_jitter;
        let row_jitter = pf.get_row_jitter() as i32 - self.pre_row_jitter;
        let voids = 0; //pf.count_voids() as i32;
        let jitter = col_jitter * 3 + row_jitter;
        return bottom_block - jitter - voids * 2;
    }
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
        PlayerCommon::new("Human 1", 500000000, figure_list.clone()),
        player1_key_map
    );
    let mut player2 = HumanPlayer::new(
        PlayerCommon::new("Human 2", 500000000, figure_list.clone()),
        player2_key_map
    );

    let mut com_type1 = JitterComputer::new();
    let mut com1 = ComputerPlayer::new(
        PlayerCommon::new("Computer 1", 100000000, figure_list.clone()),
        90000000, &mut com_type1,
    );
    let mut com_random2 = RandomComputer{};
    let mut com2 = ComputerPlayer::new(
        PlayerCommon::new("Computer 1", 5000000, figure_list.clone()),
        5000000, &mut com_random2,
    );

    let pf1 = Playfield::new("Playfield 1",
                             PF_WIDTH as usize, PF_HEIGHT as usize);
    let mut pf_ctx = PlayfieldContext::new(pf1);


//    pf_ctx.add_player(&mut player1);
//    pf_ctx.add_player(&mut player2);
    pf_ctx.add_player(&mut com1);
//    pf_ctx.add_player(&mut com2);
//    pf_ctx.add_player(&mut com3);

    let mut pause = false;
    let mut frame_cnt_sec = 0;
    let mut sec_timer = 0;
    let mut pressed_keys: HashMap<Keycode, u64> = HashMap::new();
    let mut events = sdl_context.event_pump().unwrap();
    let start_ticks = time::precise_time_ns();
    'running: loop {
        let ticks = time::precise_time_ns() - start_ticks;
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
                        pressed_keys.insert(key, ticks);
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

            player.update(ticks, &pf_ctx.pf);
            player.handle_input(ticks, &mut pressed_keys);

            if player.figure_in_play() {
                // Player has a figure in game
                let move_and_time = player.common_mut().get_next_move(ticks);
                if let Some(move_and_time) = move_and_time {
                    player.handle_move(&mut pf_ctx.pf, move_and_time);
                    if !player.figure_in_play() {
                        let scan: Vec<usize> =
                            (0..pf_ctx.pf.height()).collect();
                        let mut locked = pf_ctx.pf.get_locked_lines(&scan);

                        pf_ctx.pf.set_lines(&locked, &Block::new_locked(10));
                        pf_ctx.lines_to_throw.append(&mut locked);
                    }
                }
            } else if pf_ctx.lines_to_throw.len() == 0 {
                if !player.place_new_figure(ticks, &mut pf_ctx.pf) {
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
            draw.draw_next_figure(&mut renderer,
                                  &player.next_figure(),
                                  PF_WIDTH + 3,
                                  (figure_max_height + 1) * pi,
                                  figure_max_width, figure_max_height);

            pi += 1;
        }
        draw.present(&mut renderer);

        /* Write FPS in window title */
        frame_cnt_sec += 1;
        if (ticks as i64 - sec_timer as i64) >= 1000000000 {
            let title = format!("RSTris (fps: {})", frame_cnt_sec);
            let mut window = renderer.window_mut().unwrap();

            frame_cnt_sec = 0;
            sec_timer = ticks;
            window.set_title(&title).unwrap();
        }
        std::thread::sleep(std::time::Duration::new(0, 100000));
    }
}
