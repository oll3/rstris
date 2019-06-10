extern crate rand;
extern crate rstris;
extern crate sdl2;
extern crate time;

mod computer_player;
mod draw;
mod game_logic;
mod human_player;
mod player;

use computer_player::*;
use draw::*;
use game_logic::*;
use human_player::*;
use player::*;
use rstris::block::*;
use rstris::figure::*;
use rstris::figure_pos::*;
use rstris::playfield::*;
use rstris::position::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashMap;

static PF_WIDTH: u32 = 16;
static PF_HEIGHT: u32 = 30;
static BLOCK_SIZE: u32 = 20;
static BLOCK_SPACING: u32 = 1;

struct PlayfieldContext<'a> {
    pf: Playfield,
    players: Vec<&'a mut Player>,
    game_over: bool,
    lines_to_throw: Vec<u32>,
}

impl<'a> PlayfieldContext<'a> {
    pub fn new(pf: Playfield) -> Self {
        PlayfieldContext {
            pf,
            players: Vec::new(),
            game_over: false,
            lines_to_throw: Vec::new(),
        }
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

macro_rules! bl {
    ($x:expr) => {
        match $x {
            0 => Block::Clear,
            _ => Block::Set($x),
        }
    };
}

//
// Build list of figures
//
fn init_figures() -> Vec<Figure> {
    let mut figure_list: Vec<Figure> = Vec::new();
    figure_list.push(Figure::new_from_face(
        "1",
        &[
            &[bl!(0), bl!(0), bl!(0)],
            &[bl!(1), bl!(1), bl!(1)],
            &[bl!(0), bl!(1), bl!(0)],
        ],
    ));
    figure_list.push(Figure::new_from_face(
        "2",
        &[
            &[bl!(0), bl!(0), bl!(0)],
            &[bl!(2), bl!(2), bl!(2)],
            &[bl!(0), bl!(0), bl!(2)],
        ],
    ));
    figure_list.push(Figure::new_from_face(
        "3",
        &[
            &[bl!(0), bl!(0), bl!(3)],
            &[bl!(3), bl!(3), bl!(3)],
            &[bl!(0), bl!(0), bl!(0)],
        ],
    ));
    figure_list.push(Figure::new_from_face(
        "4",
        &[&[bl!(4), bl!(4)], &[bl!(4), bl!(4)]],
    ));
    figure_list.push(Figure::new_from_face(
        "5",
        &[&[bl!(0), bl!(5), bl!(5)], &[bl!(5), bl!(5), bl!(0)]],
    ));
    figure_list.push(Figure::new_from_face(
        "6",
        &[&[bl!(6), bl!(6), bl!(0)], &[bl!(0), bl!(6), bl!(6)]],
    ));
    figure_list.push(Figure::new_from_face(
        "7",
        &[
            &[bl!(0), bl!(7), bl!(0)],
            &[bl!(0), bl!(7), bl!(0)],
            &[bl!(0), bl!(7), bl!(0)],
            &[bl!(0), bl!(7), bl!(0)],
        ],
    ));
    return figure_list;
}

fn get_max_figure_dimensions(figure_list: &[Figure]) -> (u32, u32) {
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;
    for fig in figure_list {
        for face in fig.faces() {
            if face.width() as u32 > max_width {
                max_width = face.width() as u32;
            }
            if face.height() as u32 > max_height {
                max_height = face.height() as u32;
            }
        }
    }
    return (max_width, max_height);
}

struct RandomComputer {}
impl ComputerType for RandomComputer {
    fn init_eval(&mut self, _: &Playfield, _: usize) {}
    fn eval_placing(&mut self, _: &FigurePos, _: &Playfield) -> f32 {
        rand::random::<f32>()
    }
}

fn lowest_block(fig_pos: &FigurePos) -> i32 {
    fig_pos.get_face().row_of_lowest_block() + fig_pos.get_position().get_y()
}

fn get_pf_row_jitter(pf: &Playfield) -> u32 {
    let mut jitter = 0;
    for y in 0..((pf.height() + 1) as i32) {
        let mut last_state = pf.block_is_set(Position::new((0, y)));
        for x in 0..(pf.width() as i32) {
            let state = pf.block_is_set(Position::new((x, y)));
            if last_state != state {
                last_state = state;
                jitter += 1;
            }
        }
    }
    return jitter;
}
fn get_pf_col_jitter(pf: &Playfield) -> u32 {
    let mut jitter = 0;
    for x in -1..((pf.width() + 1) as i32) {
        let mut last_state = pf.block_is_set(Position::new((x, 0)));
        for y in 0..(pf.height() as i32) {
            let state = pf.block_is_set(Position::new((x, y)));
            if last_state != state {
                last_state = state;
                jitter += 1;
            }
        }
    }
    return jitter;
}
fn get_pf_avg_height(pf: &Playfield) -> f32 {
    let mut total_height = 0;
    for x in 0..(pf.width() as i32) {
        for y in 0..(pf.height() as i32) {
            if pf.block_is_set(Position::new((x, y))) {
                total_height += pf.height() as i32 - y;
                break;
            }
        }
    }
    return total_height as f32 / pf.width() as f32;
}
fn get_pf_max_height(pf: &Playfield) -> i32 {
    let mut max_height = 0;
    for x in 0..(pf.width() as i32) {
        for y in 0..(pf.height() as i32) {
            let height = pf.height() as i32 - y;
            if pf.block_is_set(Position::new((x, y))) && height > max_height {
                max_height = height;
            }
        }
    }
    return max_height;
}

struct JitterComputer {
    pre_col_jitter: i32,
    pre_row_jitter: i32,
    pre_voids: i32,
    pre_avg_height: f32,
    avg_height_factor: f32,
    pre_max_height: i32,
    pre_locked_lines: i32,
}
impl JitterComputer {
    fn new() -> Self {
        JitterComputer {
            pre_col_jitter: 0,
            pre_row_jitter: 0,
            pre_voids: 0,
            pre_avg_height: 0.0,
            avg_height_factor: 0.0,
            pre_max_height: 0,
            pre_locked_lines: 0,
        }
    }
}
impl ComputerType for JitterComputer {
    fn init_eval(&mut self, pf: &Playfield, _: usize) {
        self.pre_voids = pf.count_voids() as i32;
        self.pre_col_jitter = get_pf_col_jitter(pf) as i32;
        self.pre_row_jitter = get_pf_row_jitter(pf) as i32;
        self.pre_avg_height = get_pf_avg_height(pf);
        self.pre_max_height = get_pf_max_height(pf);
        self.avg_height_factor = self.pre_avg_height / pf.height() as f32;
        self.pre_locked_lines = pf.count_locked_lines() as i32;
    }

    fn eval_placing(&mut self, fig_pos: &FigurePos, pf: &Playfield) -> f32 {
        let mut pf = pf.clone();
        fig_pos.place(&mut pf);
        let mut locked_lines = pf.locked_lines();
        let _lock_cnt = locked_lines.len() as i32 - self.pre_locked_lines;
        //let tetris = if lock_cnt >= 4 {1000.0} else {0.0};
        locked_lines.sort();
        for line in &locked_lines {
            pf.throw_line(*line);
        }
        /*let avg_height = get_pf_avg_height(&pf);
        let remove_lines_score = lock_cnt as f32 * 1000.0 *
            (self.avg_height_factor - 0.24);*/

        let bottom_block = lowest_block(fig_pos);
        // Measure playfield jitter - The lower jitter the better
        let col_jitter = get_pf_col_jitter(&pf) as i32 - self.pre_col_jitter;
        let row_jitter = get_pf_row_jitter(&pf) as i32 - self.pre_row_jitter;
        let voids = 0; //pf.count_voids() as i32 - self.pre_voids;
        let jitter = col_jitter * 2 + row_jitter;
        let total_score =
            (bottom_block - jitter - voids * 4) as f32 /* +
            remove_lines_score + tetris*/;
        return total_score;
    }
}

fn main() {
    let frame_color = Color::RGB(200, 64, 64);
    let fill_color = Color::RGB(98, 204, 244);
    let bg_color = Color::RGB(101, 208, 246);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let figure_list = init_figures();
    let (figure_max_width, figure_max_height) = get_max_figure_dimensions(&figure_list);
    println!(
        "Max figure dimensions: {} x {}",
        figure_max_width, figure_max_height
    );

    let window_width: u32 = (PF_WIDTH + 2 + figure_max_width + 3) * (BLOCK_SIZE + BLOCK_SPACING);
    let window_height: u32 = (PF_HEIGHT + 1) * (BLOCK_SIZE + BLOCK_SPACING);
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut draw = DrawContext::new(BLOCK_SIZE, BLOCK_SPACING, frame_color, fill_color);

    let player1_key_map = KeyMap {
        step_left: Some(Keycode::Left),
        step_right: Some(Keycode::Right),
        step_down: Some(Keycode::Down),
        rot_cw: Some(Keycode::Up),
        rot_ccw: None,
    };

    let player2_key_map = KeyMap {
        step_left: Some(Keycode::A),
        step_right: Some(Keycode::D),
        step_down: Some(Keycode::S),
        rot_cw: Some(Keycode::W),
        rot_ccw: None,
    };
    let _player1 = HumanPlayer::new(
        PlayerCommon::new("Human 1", 500000000, figure_list.clone()),
        player1_key_map,
    );
    let _player2 = HumanPlayer::new(
        PlayerCommon::new("Human 2", 500000000, figure_list.clone()),
        player2_key_map,
    );

    let mut com_type1 = JitterComputer::new();
    let mut com1 = ComputerPlayer::new(
        PlayerCommon::new("Computer 1", 4000000, figure_list.clone()),
        4000000,
        &mut com_type1,
    );
    let mut com_random2 = RandomComputer {};
    let _com2 = ComputerPlayer::new(
        PlayerCommon::new("Computer 1", 5000000, figure_list.clone()),
        5000000,
        &mut com_random2,
    );

    let pf1 = Playfield::new("Playfield 1", PF_WIDTH, PF_HEIGHT);
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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    pause = !pause;
                    if pause {
                        println!("Paused");
                    } else {
                        println!("Continued");
                    }
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    pressed_keys.entry(key).or_insert(ticks);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if pressed_keys.contains_key(&key) {
                        pressed_keys.remove(&key);
                    }
                }

                _ => {}
            }
        }

        if pause || pf_ctx.game_over {
            continue;
        }

        // Handle movement and figure creation
        for player in &mut pf_ctx.players {
            player.update(ticks, &pf_ctx.pf);
            player.handle_input(ticks, &mut pressed_keys);

            if player.figure_in_play() {
                // Player has a figure in game
                let move_and_time = player.common_mut().get_next_move(ticks);
                if let Some(move_and_time) = move_and_time {
                    execute_move(*player, &mut pf_ctx.pf, move_and_time);
                    if !player.figure_in_play() {
                        let mut locked = pf_ctx.pf.locked_lines();
                        pf_ctx.pf.set_lines(&locked, &Block::Set(10));
                        pf_ctx.lines_to_throw.append(&mut locked);
                    }
                }
            } else if pf_ctx.lines_to_throw.is_empty() {
                let placement_result = try_place_new_figure(*player, ticks, &mut pf_ctx.pf);
                if placement_result {
                    pf_ctx.game_over = true;
                }
            }
        }

        // Throw full lines (when there are no figures being played)
        if !pf_ctx.lines_to_throw.is_empty() && !pf_ctx.figures_in_play() {
            pf_ctx.lines_to_throw.sort();
            pf_ctx.lines_to_throw.dedup();
            println!("Throw away lines: {:?}", pf_ctx.lines_to_throw);
            for line in &pf_ctx.lines_to_throw {
                pf_ctx.pf.throw_line(*line);
            }
            pf_ctx.lines_to_throw.clear();
        }

        // Render graphics
        draw.clear(&mut canvas, bg_color);
        draw.draw_playfield(&mut canvas, &pf_ctx.pf);
        for (pi, player) in pf_ctx.players.iter().enumerate() {
            draw.draw_next_figure(
                &mut canvas,
                &player.next_figure(),
                (PF_WIDTH + 3) as i32,
                ((figure_max_height + 1) * pi as u32) as i32,
                figure_max_width as i32,
                figure_max_height as i32,
            );
        }
        draw.present(&mut canvas);

        // Write FPS in window title
        frame_cnt_sec += 1;
        if (ticks as i64 - sec_timer as i64) >= 1000000000 {
            let title = format!("RSTris (fps: {})", frame_cnt_sec);
            let window = canvas.window_mut();

            frame_cnt_sec = 0;
            sec_timer = ticks;
            window.set_title(&title).unwrap();
        }
        std::thread::sleep(std::time::Duration::new(0, 100000));
    }
}
