extern crate fern;
extern crate log;

use log::*;

use sdl2;
use time;

mod computer_player;
mod draw;

mod game;

use crate::game::*;

use crate::computer_player::*;
use crate::draw::*;
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
static BLOCK_SIZE: u32 = 16;
static BLOCK_SPACING: u32 = 1;

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
#[allow(clippy::cognitive_complexity)]
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
    figure_list
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
    (max_width, max_height)
}

fn get_pf_row_jitter(pf: &Playfield) -> u32 {
    let mut jitter = 0;
    for y in 0..(pf.height() as i32) {
        let mut last_state = pf.block_is_set(Position::new((-1, y)));
        for x in 0..=(pf.width() as i32) {
            let state = pf.block_is_set(Position::new((x, y)));
            if last_state != state {
                last_state = state;
                jitter += 1;
            }
        }
    }
    jitter
}
fn get_pf_col_jitter(pf: &Playfield) -> u32 {
    let mut jitter = 0;
    for x in 0..(pf.width() as i32) {
        let mut last_state = pf.block_is_set(Position::new((x, 0)));
        for y in 0..((pf.height() + 1) as i32) {
            let state = pf.block_is_set(Position::new((x, y)));
            if last_state != state {
                last_state = state;
                jitter += 1;
            }
        }
    }
    jitter
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
    total_height as f32 / pf.width() as f32
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
    max_height
}

struct JitterComputer {
    pre_col_jitter: i32,
    pre_row_jitter: i32,
    pre_voids: i32,
    pre_avg_height: f32,
    avg_height_factor: f32,
    pre_max_height: i32,
    pre_locked_lines: i32,
    pf: Option<Playfield>,
}
impl JitterComputer {
    fn new() -> Self {
        JitterComputer {
            pf: None,
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
        if self.pf.is_none() {
            self.pf = Some(pf.clone())
        }
        self.pre_voids = pf.count_voids() as i32;
        self.pre_col_jitter = get_pf_col_jitter(pf) as i32;
        self.pre_row_jitter = get_pf_row_jitter(pf) as i32;
        self.pre_avg_height = get_pf_avg_height(pf);
        self.pre_max_height = get_pf_max_height(pf);
        self.avg_height_factor = self.pre_avg_height / pf.height() as f32;
        self.pre_locked_lines = pf.count_locked_lines() as i32;
    }

    fn eval_placing(&mut self, fig_pos: &FigurePos, current_pf: &Playfield) -> f32 {
        if let Some(ref mut pf) = self.pf {
            pf.copy(current_pf);
            fig_pos.place(pf);
            let avg_height = get_pf_avg_height(&pf);
            let mut full_lines = pf.locked_lines();
            full_lines.sort();

            let full_lines_score = if full_lines.len() >= 4 {
                // Great things!
                10.0
            } else if full_lines.len() == 1 {
                // Single full line - Not too bad but still a bit unnecessary
                -2.0
            } else if full_lines.len() >= 2 {
                // 2 or 3 lines should be avoided as long as the avarage playfield height is low
                let factor = 1.0 - (avg_height as f32 / PF_HEIGHT as f32);
                (4 - full_lines.len()) as f32 * -factor * 3.0
            } else {
                // No full lines - Don't care
                0.0
            };

            for line in &full_lines {
                pf.throw_line(*line);
            }

            let bottom_block = fig_pos.lowest_block() / 2;

            // Measure playfield jitter. Lower jitter is better.
            let col_jitter = get_pf_col_jitter(&pf) as i32 - self.pre_col_jitter;
            let row_jitter = get_pf_row_jitter(&pf) as i32 - self.pre_row_jitter;
            let jitter_score = -(col_jitter * 3 + row_jitter / 2);

            (bottom_block + jitter_score) as f32 + full_lines_score
        } else {
            0.0
        }
    }
}

fn init_log(level: log::LevelFilter) {
    let local_level = level;
    fern::Dispatch::new()
        .format(move |out, message, record| {
            if local_level > log::LevelFilter::Info {
                // Add some extra info to each message in debug
                out.finish(format_args!(
                    "[{}]({})({}) {}",
                    chrono::Local::now().format("%H:%M:%S%.6f"), //%Y-%m-%dT
                    record.target(),
                    record.level(),
                    message
                ))
            } else {
                out.finish(format_args!("{}", message))
            }
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
        .expect("unable to initialize log");
}

fn main() {
    init_log(log::LevelFilter::Debug);

    let frame_color = Color::RGB(200, 64, 64);
    let fill_color = Color::RGB(98, 204, 244);
    let bg_color = Color::RGB(101, 208, 246);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let figure_list = init_figures();
    let (figure_max_width, figure_max_height) = get_max_figure_dimensions(&figure_list);
    info!(
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

    let mut com1 = ComputerPlayer::new(1.0, JitterComputer::new());

    let mut game = Game::new(
        Playfield::new("Playfield 1", PF_WIDTH, PF_HEIGHT),
        figure_list.clone(),
        10_000_000,
    );

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
                        info!("Paused");
                    } else {
                        info!("Continued");
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

        if pause || game.game_is_over() {
            continue;
        }

        com1.act_on_game(&mut game, ticks);
        game.update(ticks);

        // Render graphics
        {
            draw.clear(&mut canvas, bg_color);
            draw.draw_playfield(&mut canvas, game.get_playfield());
            if let Some(ref current_figure) = game.get_current_figure() {
                draw.draw_figure(&mut canvas, current_figure);
            }
            draw.draw_next_figure(
                &mut canvas,
                game.get_next_figure(),
                (PF_WIDTH + 3) as i32,
                (figure_max_height + 1) as i32,
                figure_max_width as i32,
                figure_max_height as i32,
            );

            draw.present(&mut canvas);
        }

        // Write FPS in window title
        frame_cnt_sec += 1;
        if (ticks as i64 - sec_timer as i64) >= 1_000_000_000 {
            let title = format!("RSTris (fps: {})", frame_cnt_sec);
            let window = canvas.window_mut();

            frame_cnt_sec = 0;
            sec_timer = ticks;
            window.set_title(&title).unwrap();
        }
        std::thread::sleep(std::time::Duration::new(0, 50_000));
    }
}
