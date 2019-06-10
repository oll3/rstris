use crate::block::Block;
use crate::figure_face::FigureFace;
use crate::playfield::Playfield;
use crate::pos_dir::PosDir;

#[derive(Clone, Debug)]
pub struct Figure {
    figure_name: String,
    vfaces: Vec<FigureFace>,
}

impl Figure {
    pub fn new(name: &str) -> Figure {
        Figure {
            figure_name: name.to_owned(),
            vfaces: vec![],
        }
    }

    //
    // Build new figure by rotating the face of a figure 90 degrees
    //
    pub fn new_from_face(name: &str, blocks: &[&[Block]]) -> Figure {
        let mut fig = Figure::new(name);
        let mut face = FigureFace::new(blocks);
        fig.vfaces.push(face.clone());
        for _ in 0..3 {
            let mut next_face = FigureFace::new_empty(face.height(), face.width());
            for y in 0..face.height() as i32 {
                for x in 0..face.width() as i32 {
                    let ty = face.height() as i32 - y - 1;
                    let b = face.get_block((x, ty).into());
                    next_face.set_block((y, x).into(), b.clone());
                }
            }
            if !fig.test_face_present(&next_face) {
                fig.vfaces.push(next_face.clone());
            }
            face = next_face;
        }
        println!(
            "Built figure {} with {} faces",
            fig.get_name(),
            fig.faces().len()
        );
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
    //
    // Remove figure from playfield
    //
    pub fn remove(&self, pf: &mut Playfield, pos: &PosDir) {
        let face = self.get_face(pos.get_dir() as usize);
        face.remove(pf, pos.get_pos());
    }

    pub fn test_collision(&self, pf: &Playfield, pos: &PosDir) -> bool {
        let face = self.get_face(pos.get_dir() as usize);
        return face.test_collision(pf, pos.get_pos());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;
    use crate::figure_face::*;

    macro_rules! bl {
        ($x:expr) => {
            match $x {
                0 => Block::Clear,
                _ => Block::Set($x),
            }
        };
    }

    #[test]
    fn test_figure1() {
        let fig = Figure::new_from_face(
            "Figure 1",
            &[
                &[bl!(0), bl!(0), bl!(0)],
                &[bl!(1), bl!(1), bl!(1)],
                &[bl!(0), bl!(1), bl!(0)],
            ],
        );
        assert_eq!(fig.faces().len(), 4);
        assert_eq!(
            *fig.get_face(0),
            FigureFace::new(&[
                &[bl!(0), bl!(0), bl!(0)],
                &[bl!(1), bl!(1), bl!(1)],
                &[bl!(0), bl!(1), bl!(0)]
            ])
        );
        assert_eq!(fig.get_face(0).rows_with_blocks(), [1, 2]);
        assert_eq!(
            *fig.get_face(1),
            FigureFace::new(&[
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(1), bl!(1), bl!(0)],
                &[bl!(0), bl!(1), bl!(0)]
            ])
        );
        assert_eq!(fig.get_face(1).rows_with_blocks(), [0, 1, 2]);
        assert_eq!(
            *fig.get_face(2),
            FigureFace::new(&[
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(1), bl!(1), bl!(1)],
                &[bl!(0), bl!(0), bl!(0)]
            ])
        );
        assert_eq!(fig.get_face(2).rows_with_blocks(), [0, 1]);
        assert_eq!(
            *fig.get_face(3),
            FigureFace::new(&[
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(0), bl!(1), bl!(1)],
                &[bl!(0), bl!(1), bl!(0)]
            ])
        );
        assert_eq!(fig.get_face(3).rows_with_blocks(), [0, 1, 2]);
    }
    #[test]
    fn test_figure2() {
        let fig = Figure::new_from_face(
            "Figure 2",
            &[
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(0), bl!(1), bl!(0)],
            ],
        );
        assert_eq!(fig.faces().len(), 2);
        assert_eq!(
            *fig.get_face(0),
            FigureFace::new(&[
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(0), bl!(1), bl!(0)],
                &[bl!(0), bl!(1), bl!(0)]
            ])
        );
        assert_eq!(fig.get_face(0).rows_with_blocks(), [0, 1, 2, 3]);
        assert_eq!(
            *fig.get_face(1),
            FigureFace::new(&[
                &[bl!(0), bl!(0), bl!(0), bl!(0)],
                &[bl!(1), bl!(1), bl!(1), bl!(1)],
                &[bl!(0), bl!(0), bl!(0), bl!(0)]
            ])
        );
        assert_eq!(fig.get_face(1).rows_with_blocks(), [1]);
    }
    #[test]
    fn test_figure3() {
        let fig = Figure::new_from_face(
            "Figure 3",
            &[&[bl!(1), bl!(0)], &[bl!(1), bl!(1)], &[bl!(0), bl!(1)]],
        );
        assert_eq!(fig.faces().len(), 2);
        assert_eq!(
            *fig.get_face(0),
            FigureFace::new(&[&[bl!(1), bl!(0)], &[bl!(1), bl!(1)], &[bl!(0), bl!(1)]])
        );
        assert_eq!(fig.get_face(0).rows_with_blocks(), [0, 1, 2]);
        assert_eq!(
            *fig.get_face(1),
            FigureFace::new(&[&[bl!(0), bl!(1), bl!(1)], &[bl!(1), bl!(1), bl!(0)]])
        );
        assert_eq!(fig.get_face(1).rows_with_blocks(), [0, 1]);
    }
}
