use crate::figure::Figure;
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
        norm_pos.normalize_dir(fig.num_faces());
        FigurePos { fig, pos: norm_pos }
    }
    pub fn get_position(&self) -> &PosDir {
        &self.pos
    }
    pub fn get_figure(&self) -> &Figure {
        &self.fig
    }
    pub fn get_face(&self) -> &[(u8, u8, u8)] {
        &self.fig.get_face(self.pos.get_dir() as usize)
    }
    pub fn set_position(&mut self, pos: &PosDir) {
        let mut norm_pos = *pos;
        norm_pos.normalize_dir(self.fig.num_faces());
        self.pos = norm_pos;
    }
    pub fn place(&self, pf: &mut Playfield) {
        self.fig.place(pf, &self.pos);
    }
    pub fn remove(&self, pf: &mut Playfield) {
        self.fig.remove(pf, &self.pos);
    }
    fn row_of_lowest_block(face: &[(u8, u8, u8)]) -> u8 {
        let mut lowest = 0;
        for (_x, y, _id) in face.iter() {
            lowest = std::cmp::max(lowest, *y);
        }
        lowest
    }
    pub fn lowest_block(&self) -> i32 {
        i32::from(Self::row_of_lowest_block(self.get_face())) + self.get_position().get_y()
    }
}
