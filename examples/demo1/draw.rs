use rstris::block::Block;
use rstris::figure::Figure;
use rstris::playfield::Playfield;
use rstris::position::Position;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct DrawContext {
    pub block_size: u32,
    pub block_spacing: u32,
    pub frame_color: Color,
    pub fill_color: Color,
}

impl DrawContext {
    pub fn new(block_size: u32, block_spacing: u32, frame_color: Color, fill_color: Color) -> Self {
        DrawContext {
            block_size,
            block_spacing,
            frame_color,
            fill_color,
        }
    }

    fn draw_block(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32, color: Color) {
        canvas.set_draw_color(color);
        let size = self.block_size as i32;
        let spacing = self.block_spacing as i32;
        let border_rect = Rect::new(
            x * size + x * spacing,
            y * size + y * spacing,
            self.block_size,
            self.block_size,
        );
        let _ignore = canvas.fill_rect(border_rect);
    }

    fn get_block_color(id: u8) -> Color {
        match id {
            1 => Color::RGB(50, 180, 50),
            2 => Color::RGB(180, 50, 50),
            3 => Color::RGB(50, 50, 180),
            4 => Color::RGB(160, 160, 100),
            5 => Color::RGB(20, 100, 100),
            6 => Color::RGB(120, 150, 0),
            7 => Color::RGB(220, 50, 140),
            10 => Color::RGB(0, 0, 0),
            _ => Color::RGB(0, 0, 0),
        }
    }

    pub fn draw_playfield(&mut self, canvas: &mut Canvas<Window>, playfield: &Playfield) {
        let frame_color = self.frame_color;
        let fill_color = self.fill_color;
        for y in 0..playfield.height() as i32 {
            self.draw_block(canvas, 0, y, frame_color);
            for x in 0..playfield.width() as i32 {
                let block = playfield.get_block((x, y).into());
                if let Block::Set(ref id) = block {
                    self.draw_block(canvas, x + 1, y, DrawContext::get_block_color(*id));
                } else {
                    self.draw_block(canvas, x + 1, y, fill_color);
                }
            }
            self.draw_block(canvas, playfield.width() as i32 + 1, y, frame_color);
        }
        for bottom in 0..(playfield.width() + 2) as i32 {
            self.draw_block(canvas, bottom, playfield.height() as i32, frame_color);
        }
    }

    pub fn draw_figure(&mut self, canvas: &mut Canvas<Window>, fig: &Figure, pos: Position) {
        for (x, y, id) in fig.face(pos.dir()) {
            self.draw_block(
                canvas,
                i32::from(*x) + 1 + pos.x(),
                i32::from(*y) + pos.y(),
                DrawContext::get_block_color(*id),
            );
        }
    }

    pub fn draw_next_figure(
        &mut self,
        canvas: &mut Canvas<Window>,
        figure: &Figure,
        offs_x: i32,
        offs_y: i32,
        fig_max_width: i32,
        fig_max_heigth: i32,
    ) {
        let frame_color = self.frame_color;
        let fill_color = self.fill_color;
        for y in 0..(fig_max_heigth + 2) {
            for x in 0..(fig_max_width + 2) {
                if y == 0 || y == (fig_max_heigth + 1) || x == 0 || x == (fig_max_width + 1) {
                    self.draw_block(canvas, x + offs_x, y + offs_y, frame_color);
                } else {
                    self.draw_block(canvas, x + offs_x, y + offs_y, fill_color);
                }
            }
        }

        let fig_x_offs = 0; //(fig_max_width - face.width() as i32) / 2;
        let fig_y_offs = 0; //(fig_max_heigth - face.height() as i32) / 2;
        for (x, y, id) in figure.face(0) {
            self.draw_block(
                canvas,
                i32::from(*x) + offs_x + 1 + fig_x_offs,
                i32::from(*y) + offs_y + 1 + fig_y_offs,
                DrawContext::get_block_color(*id),
            );
        }
    }
    pub fn clear(&mut self, canvas: &mut Canvas<Window>, color: Color) {
        let _ = canvas.set_draw_color(color);
        let _ = canvas.clear();
    }

    pub fn present(&mut self, canvas: &mut Canvas<Window>) {
        let _ = canvas.present();
    }
}
