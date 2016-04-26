extern crate sdl2;
extern crate rand;
extern crate time;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


struct Figure {
    blocks: [[[u8; 4]; 4]; 4]
}

/*
struct RSTrisLiveFigure {
    fig: i32,
    x: i32,
    y: i32,
    dir: i32,
}*/

struct RSTrisPlayfield {
    pf_width: usize,
    pf_height: usize,
    playfield: Vec<Vec<u8>>,
}

struct RSTris {
    figures: Vec<Figure>,
    pf: RSTrisPlayfield,
//    current_figure: RSTrisLiveFigure,
    current_figure: i32,
    fig_dir: i32,
    fig_x_pos: i32,
    fig_y_pos: i32,
}

impl RSTrisPlayfield {
    pub fn new(width: usize, height: usize) -> RSTrisPlayfield {
        let mut pf = RSTrisPlayfield{pf_width: width,
                                     pf_height: height,
                                     playfield: vec![]};
        for _ in 0..height {
            pf.playfield.push(vec![0; width as usize]);
        }
        pf
    }
    pub fn get_block(&self, x: usize, y: usize) -> u8 {
        self.playfield[y][x]
    }
    pub fn width(&self) -> usize {
        self.pf_width
    }
    pub fn height(&self) -> usize {
        self.pf_height
    }

    fn place_figure(&mut self, fig: &Figure, dir: i32, x: i32, y: i32) {
        for row in 0..4 {
            for col in 0..4 {
                let b = fig.blocks[dir as usize][row][col];
                if b != 0 {
                    self.playfield[y as usize + row][x as usize + col] = b;
                }
            }
        }
    }

    fn rm_figure(&mut self, fig: &Figure, dir: i32, x: i32, y: i32) {
        for row in 0..4 {
            for col in 0..4 {
                let b = fig.blocks[dir as usize][row][col];
                if b != 0 {
                    self.playfield[y as usize + row][x as usize + col] = 0;
                }
            }
        }
    }

    fn test_figure(&self, fig: &Figure, dir: i32, x: i32, y: i32) -> bool {
        for row in 0..4 {
            let offs_y  = y + row;
            for col in 0..4 {
                let offs_x  = x + col;
                let b = fig.blocks[dir as usize][row as usize][col as usize];
                if b != 0 {
                    if offs_y < 0 || offs_y >= self.pf_height as i32 {
                        return false;
                    }
                    if offs_x < 0 || offs_x >= self.pf_width as i32 {
                        return false;
                    }
                    if self.playfield[offs_y as usize][offs_x as usize] != 0 {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn move_figure(&mut self, fig: &Figure, dir: i32, x: i32, y: i32,
                   x_offs: i32, y_offs: i32) -> bool {
        self.rm_figure(fig, dir, x, y);
        if self.test_figure(fig, dir, x + x_offs, y + y_offs) {
            self.place_figure(fig, dir, x + x_offs, y + y_offs);
            return true;
        }
        self.place_figure(fig, dir, x, y);
        return false;
    }
}

impl RSTris {

    pub fn new(width: usize, height: usize) -> RSTris {
        let tris = RSTris{figures: vec![],
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

    fn update(&mut self) {
        if self.current_figure == -1 {
            self.current_figure =
                (rand::random::<u8>() % self.figures.len() as u8) as i32;
            self.fig_dir = 0;
            self.fig_x_pos = (self.pf.pf_width / 2 - 1) as i32;
            self.fig_y_pos = 0;
            let fig = &self.figures[self.current_figure as usize];
            if self.pf.test_figure(fig, self.fig_dir, self.fig_x_pos,
                                   self.fig_y_pos) {
                self.pf.place_figure(fig, self.fig_dir,
                                     self.fig_x_pos, self.fig_y_pos);
            }
            else {
                // Game over?
                self.current_figure = -2;
                println!("Game Over!");
            }
        }
        else if self.current_figure >= 0 {
            if self.pf.move_figure(&self.figures[self.current_figure as usize],
                                   self.fig_dir, self.fig_x_pos,
                                   self.fig_y_pos,
                                   0, 1) {
                self.fig_y_pos += 1;
            }
            else {
                // Figure couldn't be moved further - Leave where ut is and
                // place another figure.
                self.current_figure = -1;
            }
        }
    }

    fn dump_figure(figure: &Figure) {
        for d in 0..4 {
            println!("Direction {}:", d);
            for y in 0..4 {
                for x in 0..4 {
                    print!("{}",
                           if figure.blocks[d][y][x] == 0 {" " } else {"X"});
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

fn draw_playfield(playfield: &RSTrisPlayfield, renderer: &mut Renderer) {
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
    for bottom in 0..(playfield.pf_width + 2) {
        draw_block(renderer, bottom as i32,
                   playfield.height() as i32, frame_color);
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
        draw_playfield(&rstris.pf, &mut renderer);
        let _ = renderer.present();
    }
}
