use crate::figure::*;
use crate::figure_face::*;
use crate::playfield::*;
use crate::pos_dir::*;

#[derive(Debug, Clone)]
pub struct FigurePos {
    fig: Figure,
    pos: PosDir,
}

impl FigurePos {
    pub fn new(fig: Figure, pos: PosDir) -> Self {
        let mut norm_pos = pos;
        norm_pos.normalize_dir(fig.faces().len());
        FigurePos { fig, pos: norm_pos }
    }
    pub fn get_position(&self) -> &PosDir {
        &self.pos
    }
    pub fn get_figure(&self) -> &Figure {
        &self.fig
    }
    pub fn get_face(&self) -> &FigureFace {
        &self.fig.get_face(self.pos.get_dir() as usize)
    }
    pub fn set_position(&mut self, pos: &PosDir) {
        let mut norm_pos = *pos;
        norm_pos.normalize_dir(self.fig.faces().len());
        self.pos = norm_pos;
    }
    pub fn place(&self, pf: &mut Playfield) {
        self.fig.place(pf, &self.pos);
    }
    pub fn remove(&self, pf: &mut Playfield) {
        self.fig.remove(pf, &self.pos);
    }
    pub fn lowest_block(&self) -> i32 {
        self.get_face().row_of_lowest_block() + self.get_position().get_y()
    }
}
