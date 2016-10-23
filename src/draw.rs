use rstris::block::*;
use rstris::playfield::*;
use rstris::figure::*;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::pixels::Color;


pub struct DrawContext {
    pub block_size: u32,
    pub block_spacing: u32,
    pub frame_color: Color,
    pub fill_color: Color,
}

impl DrawContext {

    pub fn new(block_size: u32, block_spacing: u32,
               frame_color: Color, fill_color: Color) -> Self {
        let ctx = DrawContext{block_size: block_size,
                              block_spacing: block_spacing,
                              frame_color: frame_color,
                              fill_color: fill_color};
        return ctx;
    }

    fn draw_block(&mut self, renderer: &mut Renderer,
                  x: i32, y: i32, color: Color) {
        renderer.set_draw_color(color);
        let size = self.block_size as i32;
        let spacing = self.block_spacing as i32;
        let border_rect =
            Rect::new(x * size + x * spacing, y * size + y * spacing,
                      self.block_size, self.block_size);
        let _ = renderer.fill_rect(border_rect);
    }

    fn get_block_color(block: &Block) -> Color {
        let mut color = match block.id {
            1 => Color::RGB(50, 180, 50),
            2 => Color::RGB(180, 50, 50),
            3 => Color::RGB(50, 50, 180),
            4 => Color::RGB(160, 160, 100),
            5 => Color::RGB(20, 100, 100),
            6 => Color::RGB(120, 150, 0),
            7 => Color::RGB(220, 50, 140),
            10 => Color::RGB(0, 0, 0),
            _ => Color::RGB(0, 0, 0),
        };
        if block.state == BlockState::Locked {
            let (r, g, b) = color.rgb();
            let grey = (r as u32 + g as u32 + b as u32) / 3;
            color = Color::RGB(grey as u8, grey as u8, grey as u8);
            /*((r as f32) * 0.5) as u8,
                               ((g as f32) * 0.5) as u8,
                               ((b as f32) * 0.5) as u8);*/
        }
        return color;
    }

    pub fn draw_playfield(&mut self, renderer: &mut Renderer,
                          playfield: &Playfield) {
        let frame_color = self.frame_color;
        let fill_color = self.fill_color;
        for y in 0..playfield.height() as i32 {
            self.draw_block(renderer, 0, y, frame_color);
            for x in 0..playfield.width() as i32 {
                let block = playfield.get_block(x, y);
                if block.is_set() {
                    self.draw_block(renderer, x + 1, y,
                                    DrawContext::get_block_color(&block));
                } else {
                    self.draw_block(renderer, x + 1, y, fill_color);
                }
            }
            self.draw_block(renderer, playfield.width() as i32 + 1,
                            y, frame_color);
        }
        for bottom in 0..(playfield.width() + 2) as i32 {
            self.draw_block(renderer, bottom, playfield.height() as i32,
                            frame_color);
        }
    }

    pub fn draw_next_figure(&mut self,
                            renderer: &mut Renderer,
                            figure: &Figure, offs_x: i32, offs_y: i32,
                            fig_max_width: i32, fig_max_heigth: i32) {
        let frame_color = self.frame_color;
        let fill_color = self.fill_color;
        for y in 0..(fig_max_heigth + 2) {
            for x in 0..(fig_max_width + 2) {
                if y == 0 || y == (fig_max_heigth + 1) ||
                    x == 0 || x == (fig_max_width + 1) {
                        self.draw_block(renderer,
                                        x + offs_x,
                                        y + offs_y,
                                        frame_color);
                    }
                else {
                    self.draw_block(renderer,
                                    x + offs_x,
                                    y + offs_y,
                                    fill_color);
                }
            }
        }

        let face = &figure.get_face(0);
        let fig_x_offs = (fig_max_width - face.get_width() as i32) / 2;
        let fig_y_offs = (fig_max_heigth - face.get_height() as i32) / 2;
        for y in 0..face.get_height() as i32 {
            for x in 0..face.get_width() as i32 {
                let block = face.get_block(x, y);
                if block.is_set() {
                    self.draw_block(renderer,
                                    x + offs_x + 1 + fig_x_offs,
                                    y + offs_y + 1 + fig_y_offs,
                                    DrawContext::get_block_color(&block));
                }
            }
        }
    }
    pub fn clear(&mut self, renderer: &mut Renderer, color: Color) {
        let _ = renderer.set_draw_color(color);
        let _ = renderer.clear();
    }

    pub fn present(&mut self, renderer: &mut Renderer) {
        let _ = renderer.present();
    }
}
