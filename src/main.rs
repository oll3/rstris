extern crate sdl2;
extern crate time;

mod rstris;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;



fn draw_block(renderer: &mut Renderer, x: i32, y: i32, color: Color) {
    let block_width: i32 = 32;
    let block_height: i32 = 32;
    let block_spacing: i32 = 2;
    renderer.set_draw_color(color);
    let border_rect = Rect::new(x * block_width + x * block_spacing,
                                y * block_height + y * block_spacing,
                                block_width as u32, block_height as u32);
    let _ = renderer.fill_rect(border_rect);
}

fn draw_playfield(playfield: &rstris::Playfield, renderer: &mut Renderer) {
    let frame_color = Color::RGB(200, 64, 64);
    for y in 0..playfield.height() {
        draw_block(renderer, 0, y as i32, frame_color);
        for x in 0..playfield.width() {
            let block = playfield.get_block(x, y);
            if block != 0 {
                draw_block(renderer, (x + 1) as i32, y as i32,
                           Color::RGB(0, 128, 128));
            }
        }
        draw_block(renderer, (playfield.width() + 1) as i32,
                   y as i32, frame_color);
    }
    for bottom in 0..(playfield.width() + 2) {
        draw_block(renderer, bottom as i32,
                   playfield.height() as i32, frame_color);
    }
}


//
// Build list of figures
//
fn init_figures() -> Vec<rstris::Figure> {
    let mut figure_list: Vec<rstris::Figure> = Vec::new();
    let mut fig1 = rstris::Figure::new(String::from("1"));
    fig1.add_direction(vec![vec![0, 0, 0],
                            vec![1, 1, 1],
                            vec![0, 1, 0]]);
    fig1.add_direction(vec![vec![0, 1, 0],
                            vec![1, 1, 0],
                            vec![0, 1, 0]]);
    fig1.add_direction(vec![vec![0, 1, 0],
                            vec![1, 1, 1],
                            vec![0, 0, 0]]);
    fig1.add_direction(vec![vec![0, 1, 0],
                            vec![0, 1, 1],
                            vec![0, 1, 0]]);
    figure_list.push(fig1);

    let mut fig2 = rstris::Figure::new(String::from("2"));
    fig2.add_direction(vec![vec![0, 0, 0],
                            vec![2, 2, 2],
                            vec![0, 0, 2],
                            vec![0, 0, 0]]);
    fig2.add_direction(vec![vec![0, 2, 0],
                            vec![0, 2, 0],
                            vec![2, 2, 0]]);
    fig2.add_direction(vec![vec![2, 0, 0],
                            vec![2, 2, 2],
                            vec![0, 0, 0]]);
    fig2.add_direction(vec![vec![0, 2, 2],
                            vec![0, 2, 0],
                            vec![0, 2, 0]]);
    figure_list.push(fig2);

    let mut fig3 = rstris::Figure::new(String::from("3"));
    fig3.add_direction(vec![vec![3, 3, 0],
                            vec![0, 3, 3],
                            vec![0, 0, 0]]);
    fig3.add_direction(vec![vec![0, 3, 0],
                            vec![3, 3, 0],
                            vec![3, 0, 0]]);
    figure_list.push(fig3);

    let mut fig4 = rstris::Figure::new(String::from("4"));
    fig4.add_direction(vec![vec![4, 4],
                            vec![4, 4]]);
    figure_list.push(fig4);

    let mut fig5 = rstris::Figure::new(String::from("5"));
    fig5.add_direction(vec![vec![0, 5, 5],
                            vec![5, 5, 0],
                            vec![0, 0, 0]]);
    fig5.add_direction(vec![vec![5, 0, 0],
                            vec![5, 5, 0],
                            vec![0, 5, 0]]);
    figure_list.push(fig5);

    let mut fig6 = rstris::Figure::new(String::from("6"));
    fig6.add_direction(vec![vec![0, 0, 0],
                            vec![6, 6, 6],
                            vec![6, 0, 0]]);
    fig6.add_direction(vec![vec![6, 6, 0],
                            vec![0, 6, 0],
                            vec![0, 6, 0]]);
    fig6.add_direction(vec![vec![0, 0, 6],
                            vec![6, 6, 6],
                            vec![0, 0, 0]]);
    fig6.add_direction(vec![vec![0, 6, 0],
                            vec![0, 6, 0],
                            vec![0, 6, 6]]);
    figure_list.push(fig6);

    let mut fig7 = rstris::Figure::new(String::from("7"));
    fig7.add_direction(vec![vec![0, 7, 0, 0],
                            vec![0, 7, 0, 0],
                            vec![0, 7, 0, 0],
                            vec![0, 7, 0, 0]]);
    fig7.add_direction(vec![vec![0, 0, 0, 0],
                            vec![7, 7, 7, 7],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]]);
    figure_list.push(fig7);
    return figure_list;
}



fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();


    let mut renderer = window.renderer().build().unwrap();
    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();

    let figure_list = init_figures();
    let mut player1 =
        rstris::Player::new(String::from("Player 1"), &figure_list);
    let mut pf1 = rstris::Playfield::new(String::from("Playfield 1"), 12, 16);

    player1.place_next_figure(&mut pf1);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_update = time::precise_time_ns();
    'running: loop {
        let mut rel_move = rstris::Position::new(0, 0, 0);
        let mut moves: Vec<rstris::Position> = vec![];
        let current_ticks = time::precise_time_ns();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {
                    keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },Event::KeyDown {
                    keycode: Some(Keycode::Left), .. } => {
                    moves.push(rstris::Position::new(-1, 0, 0));
                },Event::KeyDown {
                    keycode: Some(Keycode::Right), .. } => {
                    moves.push(rstris::Position::new(1, 0, 0));
                },Event::KeyDown {
                    keycode: Some(Keycode::Down), .. } => {
                    moves.push(rstris::Position::new(0, 1, 0));
                    last_update = current_ticks;
                }, Event::KeyDown {
                    keycode: Some(Keycode::Up), .. } => {
                    moves.push(rstris::Position::new(0, 0, 1));
                },
                _ => {}
            }
        }

        if (last_update + 500000000) < current_ticks {
            last_update = current_ticks;
            moves.push(rstris::Position::new(0, 1, 0));
        }
        for fig_move in moves {
            if !player1.move_figure(&mut pf1, &fig_move) {
                if !player1.place_next_figure(&mut pf1) {
                    println!("Game over!");
                }
            }
        }

        /* Render graphics */
        let _ = renderer.set_draw_color(Color::RGB(101, 208, 246));
        let _ = renderer.clear();
        draw_playfield(&pf1, &mut renderer);
        let _ = renderer.present();
    }
}
