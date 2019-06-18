use crate::block::Block;
use crate::figure::Figure;
use crate::matrix2::Matrix2;
use crate::playfield::Playfield;
use crate::pos_dir::PosDir;

#[derive(PartialEq, Debug, Clone)]
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
    pub fn get_face(&self) -> &Matrix2<Block> {
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
    fn row_of_lowest_block(face: &Matrix2<Block>) -> i32 {
        let mut lowest = i32::min_value();
        face.row_iter().for_each(|row| {
            if row.items.iter().any(|b| b.is_set()) && row.point > lowest {
                lowest = row.point;
            }
        });
        lowest
    }
    pub fn lowest_block(&self) -> i32 {
        Self::row_of_lowest_block(self.get_face()) + self.get_position().get_y()
    }
}
