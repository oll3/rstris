use position::*;
use figure::*;
use figure_dir::*;
use playfield::*;

#[derive(Debug, Clone)]
pub struct FigurePos {
    fig: Figure,
    pos: Position,
}

impl FigurePos {
    pub fn new(fig: Figure, pos: Position) -> Self {
        let mut norm_pos = pos;
        norm_pos.normalize_dir(fig.dir.len());
        return FigurePos{fig: fig, pos: norm_pos};
    }
    pub fn get_position(&self) -> &Position {
        &self.pos
    }
    pub fn get_figure(&self) -> &Figure {
        &self.fig
    }
    pub fn get_figure_dir(&self) -> &FigureDir {
        &self.fig.dir[self.pos.get_dir() as usize]
    }
    pub fn set_position(&mut self, pos: &Position) {
        let mut norm_pos = pos.clone();
        norm_pos.normalize_dir(self.fig.dir.len());
        self.pos = norm_pos;
    }
    pub fn place(&self, pf: &mut Playfield) {
        self.fig.place(pf, &self.pos);
    }
    pub fn lock(&self, pf: &mut Playfield) {
        self.fig.lock(pf, &self.pos);
    }
    pub fn remove(&self, pf: &mut Playfield) {
        self.fig.remove(pf, &self.pos);
    }
}
