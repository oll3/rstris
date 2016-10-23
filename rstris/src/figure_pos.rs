use position::*;
use pos_dir::*;
use figure::*;
use figure_face::*;
use playfield::*;

#[derive(Debug, Clone)]
pub struct FigurePos {
    fig: Figure,
    pos: PosDir,
}

impl FigurePos {
    pub fn new(fig: Figure, pos: PosDir) -> Self {
        let mut norm_pos = pos;
        norm_pos.normalize_dir(fig.faces().len());
        return FigurePos{fig: fig, pos: norm_pos};
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
        let mut norm_pos = pos.clone();
        norm_pos.normalize_dir(self.fig.faces().len());
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
