use figure_face::*;
use playfield::*;
use position::*;
use block::*;


#[derive(Clone, Debug)]
pub struct Figure {
    figure_name: String,
    vfaces: Vec<FigureFace>,
}


impl Figure {
    pub fn new(name: &str) -> Figure {
        Figure{figure_name: name.to_owned(), vfaces: vec![]}
    }

    //
    // Build new figure by rotating the face of a figure 90 degrees
    //
    pub fn new_from_face(name: &str, blocks: &[&[Block]]) -> Figure {
        let mut fig = Figure::new(name);
        let mut face = FigureFace::new(blocks);
        fig.vfaces.push(face.clone());
        for _ in 0..3 {
            let mut next_face =
                FigureFace::new_empty(face.get_height(), face.get_width());
            for y in 0..face.get_height() {
                for x in 0..face.get_width() {
                    let b = &face.get_block(x, face.get_height() - y - 1);
                    next_face.set_block(y, x, b);
                }
            }
            if !fig.test_face_present(&next_face) {
                fig.vfaces.push(next_face.clone());
            }
            face = next_face;
        }
        println!("Built figure {} with {} faces",
                 fig.get_name(), fig.faces().len());
        fig
    }
    fn test_face_present(&self, face: &FigureFace) -> bool {
        for f in &self.vfaces {
            if *f == *face {
                return true;
            }
        }
        return false;
    }
    pub fn get_name(&self) -> &String {
        &self.figure_name
    }
    pub fn faces(&self) -> &Vec<FigureFace> {
        &self.vfaces
    }
    pub fn get_face(&self, face_index: usize) -> &FigureFace {
        let face_index = face_index % self.vfaces.len();
        return &self.vfaces[face_index];
    }

    //
    // Place figure in playfield
    //
    pub fn place(&self, pf: &mut Playfield, pos: &PosDir) {
        let face = self.get_face(pos.get_dir() as usize);
        face.place(pf, pos.get_pos());
    }
    pub fn lock(&self, pf: &mut Playfield, pos: &PosDir) {
        let face = self.get_face(pos.get_dir() as usize);
        face.lock(pf, pos.get_pos());
    }
    //
    // Remove figure from playfield
    //
    pub fn remove(&self, pf: &mut Playfield, pos: &PosDir) {
        let face = self.get_face(pos.get_dir() as usize);
        face.remove(pf, pos.get_pos());
    }

    pub fn test_collision(&self, pf: &Playfield, pos: &PosDir) -> BlockState {
        let face = self.get_face(pos.get_dir() as usize);
        return face.test_collision(pf, pos.get_pos());
    }

    //
    // Test if figure will collide with any locked block if placed
    // at the given position
    //
    pub fn collide_locked(&self, pf: &Playfield, pos: &PosDir) -> bool {
        self.test_collision(pf, pos) == BlockState::Locked
    }

    //
    // Test if figure will collide with any block if placed at the given
    // position.
    //
    pub fn collide_any(&self, pf: &Playfield, pos: &PosDir) -> bool {
        self.test_collision(pf, pos) != BlockState::NotSet
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use figure_face::*;
    use block::*;

    macro_rules! bl {
    ($x:expr) => (match $x {
        0 => Block::new_not_set(),
        _ => Block::new_locked($x)})
    }

    #[test]
    fn test_figure1() {
        let fig = Figure::new_from_face("Figure 1",
                                        &[&[bl!(0), bl!(0), bl!(0)],
                                          &[bl!(1), bl!(1), bl!(1)],
                                          &[bl!(0), bl!(1), bl!(0)]]);
        assert_eq!(fig.faces().len(), 4);
        assert_eq!(*fig.get_face(0),
                   FigureFace::new(&[&[bl!(0), bl!(0), bl!(0)],
                                     &[bl!(1), bl!(1), bl!(1)],
                                     &[bl!(0), bl!(1), bl!(0)]]));
        assert_eq!(fig.get_face(0).get_row_with_blocks(), [1, 2]);
        assert_eq!(*fig.get_face(1),
                   FigureFace::new(&[&[bl!(0), bl!(1), bl!(0)],
                                     &[bl!(1), bl!(1), bl!(0)],
                                     &[bl!(0), bl!(1), bl!(0)]]));
        assert_eq!(fig.get_face(1).get_row_with_blocks(), [0, 1, 2]);
        assert_eq!(*fig.get_face(2),
                   FigureFace::new(&[&[bl!(0), bl!(1), bl!(0)],
                                     &[bl!(1), bl!(1), bl!(1)],
                                     &[bl!(0), bl!(0), bl!(0)]]));
        assert_eq!(fig.get_face(2).get_row_with_blocks(), [0, 1]);
        assert_eq!(*fig.get_face(3),
                   FigureFace::new(&[&[bl!(0), bl!(1), bl!(0)],
                                     &[bl!(0), bl!(1), bl!(1)],
                                     &[bl!(0), bl!(1), bl!(0)]]));
        assert_eq!(fig.get_face(3).get_row_with_blocks(), [0, 1, 2]);
    }
    #[test]
    fn test_figure2() {
        let fig = Figure::new_from_face("Figure 2",
                                        &[&[bl!(0), bl!(1), bl!(0)],
                                          &[bl!(0), bl!(1), bl!(0)],
                                          &[bl!(0), bl!(1), bl!(0)],
                                          &[bl!(0), bl!(1), bl!(0)]]);
        assert_eq!(fig.faces().len(), 2);
        assert_eq!(*fig.get_face(0),
                   FigureFace::new(&[&[bl!(0), bl!(1), bl!(0)],
                                     &[bl!(0), bl!(1), bl!(0)],
                                     &[bl!(0), bl!(1), bl!(0)],
                                     &[bl!(0), bl!(1), bl!(0)]]));
        assert_eq!(fig.get_face(0).get_row_with_blocks(), [0, 1, 2, 3]);
        assert_eq!(*fig.get_face(1),
                   FigureFace::new(&[&[bl!(0), bl!(0), bl!(0), bl!(0)],
                                     &[bl!(1), bl!(1), bl!(1), bl!(1)],
                                     &[bl!(0), bl!(0), bl!(0), bl!(0)]]));
        assert_eq!(fig.get_face(1).get_row_with_blocks(), [1]);
    }
    #[test]
    fn test_figure3() {
        let fig = Figure::new_from_face("Figure 3",
                                        &[&[bl!(1), bl!(0)],
                                          &[bl!(1), bl!(1)],
                                          &[bl!(0), bl!(1)]]);
        assert_eq!(fig.faces().len(), 2);
        assert_eq!(*fig.get_face(0),
                   FigureFace::new(&[&[bl!(1), bl!(0)],
                                     &[bl!(1), bl!(1)],
                                     &[bl!(0), bl!(1)]]));
        assert_eq!(fig.get_face(0).get_row_with_blocks(), [0, 1, 2]);
        assert_eq!(*fig.get_face(1),
                   FigureFace::new(&[&[bl!(0), bl!(1), bl!(1)],
                                     &[bl!(1), bl!(1), bl!(0)]]));
        assert_eq!(fig.get_face(1).get_row_with_blocks(), [0, 1]);
    }
}
