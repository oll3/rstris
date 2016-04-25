extern crate sdl2;
extern crate rand;
extern crate time;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::Rng;


struct Figure {
    blocks: [[[u8; 4]; 4]; 4]
}

struct RSTrisLiveFigure {
    fig: i32,
    x: i32,
    y: i32,
    dir: i32,
}

struct RSTrisPlayfield {
    pf_width: u32,
    pf_height: u32,
    playfield: Vec<Vec<u8>>,
}

struct RSTris {
    figures: Vec<Figure>,
    pf: RSTrisPlayfield,
/*
    pf_width: u32,
    pf_height: u32,
    playfield: Vec<Vec<u8>>,
*/
//    current_figure: RSTrisLiveFigure,
    current_figure: i32,
    fig_dir: i32,
    fig_x_pos: i32,
    fig_y_pos: i32,
}

impl RSTrisPlayfield {
    pub fn new(width: u32, height: u32) -> RSTrisPlayfield {
        let mut pf = RSTrisPlayfield{pf_width: width,
                                     pf_height: height,
                                     playfield: vec![]};
        for h in 0..height {
            pf.playfield.push(vec![0; width as usize]);
        }
        pf
    }

    fn draw_block(renderer: &mut sdl2::render::Renderer,
                  x: i32, y: i32, color: Color) {
        let block_width: i32 = 32;
        let block_height: i32 = 32;
        let block_spacing: i32 = 2;
        renderer.set_draw_color(color);
        let border_rect = Rect::new(x * block_width + x * block_spacing,
                                    y * block_height + y * block_spacing,
                                    block_width as u32, block_height as u32);
        let _ = renderer.fill_rect(border_rect);
    }

    fn draw(&self, renderer: &mut sdl2::render::Renderer) {
        let mut y: i32 = 0;
        let frame_color = Color::RGB(200, 64, 64);
        for row in &self.playfield {
            let mut x: i32 = 0;
            RSTrisPlayfield::draw_block(renderer, x, y, frame_color);
            for col in row {
                if *col != 0 {
                    RSTrisPlayfield::draw_block(renderer, x + 1, y,
                                                Color::RGB(0, 128, 128));
                }
                x += 1;
            }
            RSTrisPlayfield::draw_block(renderer, x + 1, y, frame_color);
            y += 1;
        }
        for bottom in 0..(self.pf_width + 2) {
            RSTrisPlayfield::draw_block(renderer, bottom as i32, y, frame_color);
        }
    }
}

impl RSTris {

    pub fn new(width: u32, height: u32) -> RSTris {
        let mut tris = RSTris{figures: vec![],
                              pf: RSTrisPlayfield::new(width, height),
                              current_figure: -1,
                              fig_dir: 0,
                              fig_x_pos: 0,
                              fig_y_pos: 0};
        tris
    }

    fn add_figure(&mut self, fig: Figure) {
        self.figures.push(fig);
    }

    fn place_figure(&mut self, fig: &Figure, dir: i32, x: i32, y: i32) {

    }

    fn rm_figure(&mut self, fig: &Figure, dir: i32, x: i32, y: i32) {

    }

    fn update(&mut self) {
        if self.current_figure == -1 {
            self.current_figure = (rand::random::<u8>() % 7) as i32;
            self.fig_dir = 0;
//            self.fig_x_pos = (self.pf_width / 2) as i32;
//            self.fig_y_pos = 0;
        }
        else {
            /* Move figure down one step */
        }
    }

    fn dump_figure(figure: &Figure) {
        for d in 0..4 {
            println!("Direction {}:", d);
            for y in 0..4 {
                for x in 0..4 {
                    print!("{}", if figure.blocks[d][y][x] == 0 {" " } else {"X"});
                }
                println!("");
            }
        }
    }

    fn dump_figures(&self) {
        for fig in &self.figures {
            RSTris::dump_figure(fig);
        }

    }
}

fn init_figures(rstris: &mut RSTris)
{
    let fig1 = Figure{blocks: [[[0, 0, 0, 0],
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

    let fig2 = Figure{blocks: [[[0, 0, 0, 0],
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

    let fig3 = Figure{blocks: [[[1, 1, 0, 0],
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

    let fig4 = Figure{blocks: [[[1, 1, 0, 0],
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

    let fig5 = Figure{blocks: [[[0, 1, 1, 0],
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

    let fig6 = Figure{blocks: [[[0, 0, 0, 0],
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

    let fig7 = Figure{blocks: [[[0, 1, 0, 0],
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

    rstris.add_figure(fig1);
    rstris.add_figure(fig2);
    rstris.add_figure(fig3);
    rstris.add_figure(fig4);
    rstris.add_figure(fig5);
    rstris.add_figure(fig6);
    rstris.add_figure(fig7);
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

    let mut rstris = RSTris::new(12, 16);
    init_figures(&mut rstris);


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_update = 0;
    'running: loop {
        let current_ticks = time::precise_time_ns();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        if (last_update + 1000000000) < current_ticks {
            last_update = current_ticks;
            rstris.update();
        }

        /* Render graphics */
        let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(101, 208, 246));
        let _ = renderer.clear();
        rstris.pf.draw(&mut renderer);
        let _ = renderer.present();
    }
}
