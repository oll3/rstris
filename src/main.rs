extern crate sdl2;
extern crate time;
extern crate rstris;

use rstris::playfield::*;
use rstris::player::*;
use rstris::figure::*;
use rstris::position::*;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;


static PF_WIDTH: u32 = 10;
static PF_HEIGHT: u32 = 20;
static BLOCK_SIZE: u32 = 32;
static BLOCK_SPACING: u32 = 2;
static FRAME_COLOR: Color = Color::RGB(200, 64, 64);
static FILL_COLOR: Color = Color::RGB(98, 204, 244);
static BG_COLOR: Color = Color::RGB(101, 208, 246);

struct PlayerStats {
    line_count: usize,
}


fn draw_block(renderer: &mut Renderer, x: u32, y: u32, color: Color) {
    let block_width: u32 = BLOCK_SIZE;
    let block_height: u32 = BLOCK_SIZE;
    let block_spacing: u32 = BLOCK_SPACING;
    renderer.set_draw_color(color);
    let border_rect = Rect::new((x * block_width + x * block_spacing) as i32,
                                (y * block_height + y * block_spacing) as i32,
                                block_width, block_height);
    let _ = renderer.fill_rect(border_rect);
}

fn get_block_color(block_id: u8) -> Color {
    match block_id {
        1 => Color::RGB(50, 180, 50),
        2 => Color::RGB(180, 50, 50),
        3 => Color::RGB(50, 50, 180),
        4 => Color::RGB(120, 120, 120),
        5 => Color::RGB(20, 80, 80),
        6 => Color::RGB(120, 150, 0),
        7 => Color::RGB(220, 50, 140),
        _ => Color::RGB(0, 0, 0),
    }
}

fn draw_playfield(playfield: &Playfield, renderer: &mut Renderer) {
    for y in 0..playfield.height() {
        draw_block(renderer, 0, y as u32, FRAME_COLOR);
        for x in 0..playfield.width() {
            let block = playfield.get_block(x, y);
            if block != 0 {
                draw_block(renderer, (x + 1) as u32, y as u32,
                           get_block_color(block));
            } else {
                draw_block(renderer, (x + 1) as u32, y as u32,
                           FILL_COLOR);
            }
        }
        draw_block(renderer, (playfield.width() + 1) as u32,
                   y as u32, FRAME_COLOR);
    }
    for bottom in 0..(playfield.width() + 2) {
        draw_block(renderer, bottom as u32,
                   playfield.height() as u32, FRAME_COLOR);
    }
}

fn draw_next_figure(figure: &Figure, offs_x: u32, offs_y: u32,
                    fig_max_width: u32, fig_max_heigth: u32,
                    renderer: &mut Renderer) {
    for y in 0..(fig_max_heigth + 2) {
        for x in 0..(fig_max_width + 2) {
            if y == 0 || y == (fig_max_heigth + 1) ||
                x == 0 || x == (fig_max_width + 1) {
                    draw_block(renderer, x as u32 + offs_x, y as u32 + offs_y,
                               FRAME_COLOR);
                }
            else {
                draw_block(renderer, x as u32 + offs_x, y as u32 + offs_y,
                           FILL_COLOR);
            }
        }
    }

    let fig_dir = &figure.dir[0];
    let fig_x_offs = (fig_max_width - fig_dir.get_width() as u32) / 2;
    let fig_y_offs = (fig_max_heigth - fig_dir.get_height() as u32) / 2;
    for y in 0..fig_dir.get_height() {
        for x in 0..fig_dir.get_width() {
            let block = fig_dir.get_block(x, y);
            if block != 0 {
                draw_block(renderer,
                           x as u32 + offs_x + 1 + fig_x_offs,
                           y as u32 + offs_y + 1 + fig_y_offs,
                           get_block_color(block));
            }
        }
    }
}


//
// Build list of figures
//
fn init_figures() -> Vec<Figure> {
    let mut figure_list: Vec<Figure> = Vec::new();
    figure_list.push(Figure::
                     new_from_face(String::from("1"),
                                   vec![vec![0, 0, 0],
                                        vec![1, 1, 1],
                                        vec![0, 1, 0]]));
    figure_list.push(Figure::
                     new_from_face(String::from("2"),
                                   vec![vec![0, 0, 0],
                                        vec![2, 2, 2],
                                        vec![0, 0, 2]]));
    figure_list.push(Figure::
                     new_from_face(String::from("3"),
                                   vec![vec![0, 0, 3],
                                        vec![3, 3, 3],
                                        vec![0, 0, 0]]));
    figure_list.push(Figure::
                     new_from_face(String::from("4"),
                                   vec![vec![4, 4],
                                        vec![4, 4]]));
    figure_list.push(Figure::
                     new_from_face(String::from("5"),
                                   vec![vec![0, 5, 5],
                                        vec![5, 5, 0]]));
    figure_list.push(Figure::
                     new_from_face(String::from("6"),
                                   vec![vec![6, 6, 0],
                                        vec![0, 6, 6]]));
    figure_list.push(Figure::
                     new_from_face(String::from("7"),
                                   vec![vec![0, 7, 0],
                                        vec![0, 7, 0],
                                        vec![0, 7, 0],
                                        vec![0, 7, 0]]));
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


fn handle_player_moves(stats: &mut PlayerStats, pf: &mut Playfield,
                       player: &mut Player, moves: Vec<Movement>) {
    for fig_move in moves {
        if !player.move_figure(pf, fig_move) {

            // Figure couldn't be moved downwards

            // Check for full lines in playfield and throw them away
            let full_lines = pf.find_full_lines();
            let num_full_lines = full_lines.len();
            for line in full_lines {
                pf.throw_line(line);
            }

            if num_full_lines > 0 {
                stats.line_count += num_full_lines;
                println!("Removed {} lines. Total: {}",
                         num_full_lines, stats.line_count);
            }

            // Place new figure in playfield
            if !player.place_next_figure(pf) {
                println!("Game over!");
            }
        }
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
    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();

    let mut player1 = Player::new(String::from("Player 1"), &figure_list);
    let mut pf1 = Playfield::new(String::from("Playfield 1"),
                                 PF_WIDTH as usize, PF_HEIGHT as usize);

    player1.place_next_figure(&mut pf1);

    let mut player1_stats = PlayerStats{line_count: 0};
    let mut pause = false;

    let mut pressed_keys: HashMap<Keycode, (u64, u64)> = HashMap::new();
    let mut events = sdl_context.event_pump().unwrap();
    let mut last_update = time::precise_time_ns();
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
                        pressed_keys.insert(key, (current_ticks, 140000000));
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

        if pause {
            continue;
        }

        let mut moves: Vec<Movement> = vec![];
        let keys = pressed_keys.clone();
        for (key, (time, this_delay)) in keys {
            if time <= current_ticks {
                let next_delay = (this_delay * 2) / 5 + 20000000;
                pressed_keys.insert(key, (current_ticks + this_delay,
                                          next_delay));
                match key {
                    Keycode::Left => {
                        moves.push(Movement::MoveLeft);
                    },Keycode::Right => {
                        moves.push(Movement::MoveRight);
                    },Keycode::Down => {
                        moves.push(Movement::MoveDown);
                        last_update = current_ticks;
                    },Keycode::Up => {
                        moves.push(Movement::RotateCW);
                    },
                    _ => {}
                }
            }
        }

        if (last_update + 500000000) < current_ticks {
            last_update = current_ticks;
            moves.push(Movement::MoveDown);
        }

        handle_player_moves(&mut player1_stats, &mut pf1,
                            &mut player1, moves);
        /* Render graphics */
        let _ = renderer.set_draw_color(BG_COLOR);
        let _ = renderer.clear();
        draw_playfield(&pf1, &mut renderer);
        draw_next_figure(&player1.get_next_figure(), PF_WIDTH + 3, 0,
                         figure_max_width, figure_max_height,
                         &mut renderer);
        let _ = renderer.present();
    }
}
