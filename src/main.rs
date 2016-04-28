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

fn init_figures(playfield: &mut rstris::Playfield)
{
    let mut fig1 = rstris::Figure::new();
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
    playfield.add_figure(fig1);

    let mut fig2 = rstris::Figure::new();
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
    playfield.add_figure(fig2);

    let mut fig3 = rstris::Figure::new();
    fig3.add_direction(vec![vec![3, 3, 0],
                            vec![0, 3, 3],
                            vec![0, 0, 0]]);
    fig3.add_direction(vec![vec![0, 3, 0],
                            vec![3, 3, 0],
                            vec![3, 0, 0]]);
    playfield.add_figure(fig3);

    let mut fig4 = rstris::Figure::new();
    fig4.add_direction(vec![vec![4, 4],
                            vec![4, 4]]);
    playfield.add_figure(fig4);

    let mut fig5 = rstris::Figure::new();
    fig5.add_direction(vec![vec![0, 5, 5],
                            vec![5, 5, 0],
                            vec![0, 0, 0]]);
    fig5.add_direction(vec![vec![5, 0, 0],
                            vec![5, 5, 0],
                            vec![0, 5, 0]]);
    playfield.add_figure(fig5);

    let mut fig6 = rstris::Figure::new();
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
    playfield.add_figure(fig6);

    let mut fig7 = rstris::Figure::new();
    fig7.add_direction(vec![vec![0, 7, 0, 0],
                            vec![0, 7, 0, 0],
                            vec![0, 7, 0, 0],
                            vec![0, 7, 0, 0]]);
    fig7.add_direction(vec![vec![0, 0, 0, 0],
                            vec![7, 7, 7, 7],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]]);
    playfield.add_figure(fig7);
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

    let mut pf1 = rstris::Playfield::new(12, 16);
    init_figures(&mut pf1);
    let mut rstris = rstris::RSTris::new();
    rstris.add_playfield(pf1);


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_update = 0;
    'running: loop {
        let current_ticks = time::precise_time_ns();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {
                    keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        if (last_update + 500000000) < current_ticks {
            last_update = current_ticks;
            rstris.update();
        }

        /* Render graphics */
        let _ = renderer.set_draw_color(Color::RGB(101, 208, 246));
        let _ = renderer.clear();
        draw_playfield(&rstris.get_playfields()[0], &mut renderer);
        let _ = renderer.present();
    }
}
