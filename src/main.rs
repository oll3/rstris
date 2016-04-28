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

fn init_figures(playfield: &mut rstris::Playfield) //rstris: &mut rstris::RSTris)
{
    let fig1 = rstris::Figure{
        blocks: [[[0, 0, 0, 0],
                  [1, 1, 1, 0],
                  [0, 1, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 0, 0],
                  [1, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 0, 0],
                  [1, 1, 1, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 0, 0],
                  [0, 1, 1, 0],
                  [0, 1, 0, 0],
                  [0, 0, 0, 0]]]};

    let fig2 = rstris::Figure{
        blocks: [[[0, 0, 0, 0],
                  [1, 1, 1, 0],
                  [0, 0, 1, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 0, 0],
                  [0, 1, 0, 0],
                  [1, 1, 0, 0],
                  [0, 0, 0, 0]],
                 [[1, 0, 0, 0],
                  [1, 1, 1, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 1, 0],
                  [0, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 0, 0, 0]]]};

    let fig3 = rstris::Figure{
        blocks: [[[1, 1, 0, 0],
                  [0, 1, 1, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 0, 0],
                  [1, 1, 0, 0],
                  [1, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[1, 1, 0, 0],
                  [0, 1, 1, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 0, 0],
                  [1, 1, 0, 0],
                  [1, 0, 0, 0],
                  [0, 0, 0, 0]]]};

    let fig4 = rstris::Figure{
        blocks: [[[1, 1, 0, 0],
                  [1, 1, 0, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[1, 1, 0, 0],
                  [1, 1, 0, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[1, 1, 0, 0],
                  [1, 1, 0, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[1, 1, 0, 0],
                  [1, 1, 0, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]]]};

    let fig5 = rstris::Figure{
        blocks: [[[0, 1, 1, 0],
                  [1, 1, 0, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[1, 0, 0, 0],
                  [1, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 1, 0],
                  [1, 1, 0, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[1, 0, 0, 0],
                  [1, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 0, 0, 0]]]};

    let fig6 = rstris::Figure{
        blocks: [[[0, 0, 0, 0],
                  [1, 1, 1, 0],
                  [1, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[1, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 0, 1, 0],
                  [1, 1, 1, 0],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 1, 1, 0],
                  [0, 0, 0, 0]]]};

    let fig7 = rstris::Figure{
        blocks: [[[0, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 1, 0, 0]],
                 [[0, 0, 0, 0],
                  [1, 1, 1, 1],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]],
                 [[0, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 1, 0, 0],
                  [0, 1, 0, 0]],
                 [[0, 0, 0, 0],
                  [1, 1, 1, 1],
                  [0, 0, 0, 0],
                  [0, 0, 0, 0]]]};

    playfield.add_figure(fig1);
    playfield.add_figure(fig2);
    playfield.add_figure(fig3);
    playfield.add_figure(fig4);
    playfield.add_figure(fig5);
    playfield.add_figure(fig6);
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
