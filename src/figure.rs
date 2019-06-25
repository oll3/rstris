use crate::block::Block;
use crate::playfield::Playfield;
use crate::pos_dir::PosDir;

#[derive(PartialEq, Clone, Debug)]
pub struct Figure {
    figure_name: String,

    // Maximum size of figure in any direction
    max_face_width: u8,

    // Each face of the figure is represented as a number of coordinates
    blocks_per_face: u8,
    num_faces: u8,
    faces: Vec<(u8, u8, u8)>,
}

impl Figure {
    pub fn new(name: &str) -> Figure {
        Figure {
            figure_name: name.to_owned(),
            faces: Vec::new(),
            blocks_per_face: 0,
            max_face_width: 0,
            num_faces: 0,
        }
    }

    //
    // Build new figure by rotating the face of a figure 90 degrees
    //
    pub fn new_from_face(name: &str, blocks: &[&[Block]]) -> Figure {
        let blocks_per_face = blocks
            .iter()
            .map(|row| row.iter().filter(|b| b.is_set()).count() as u8)
            .sum();

        println!("Figure {} - blocks_per_face={}", name, blocks_per_face);
        let mut face1 = Vec::new();
        let mut face2 = Vec::new();
        let mut face3 = Vec::new();
        let mut face4 = Vec::new();
        // Direction 1
        for (row, row_blocks) in blocks.iter().enumerate() {
            for col in 0..row_blocks.len() {
                let block = &row_blocks[col];
                if let Block::Set(ref id) = block {
                    face1.push((col as u8, row as u8, *id));
                }
            }
        }
        // Direction 2
        for row in 0..blocks[0].len() {
            for (col, col_blocks) in blocks.iter().enumerate() {
                let block = &col_blocks[row];
                if let Block::Set(ref id) = block {
                    face2.push((col as u8, row as u8, *id));
                }
            }
        }
        // Direction 3
        for row in 0..blocks.len() {
            for col in 0..blocks[row].len() {
                let block = &blocks[blocks.len() - (row + 1)][blocks[row].len() - (col + 1)];
                if let Block::Set(ref id) = block {
                    face3.push((col as u8, row as u8, *id));
                }
            }
        }
        // Direction 4
        for row in 0..blocks[0].len() {
            for col in 0..blocks.len() {
                let block = &blocks[blocks.len() - (col + 1)][blocks[0].len() - (row + 1)];
                if let Block::Set(ref id) = block {
                    face4.push((col as u8, row as u8, *id));
                }
            }
        }

        // Remove duplicated faces
        let mut faces = Vec::new();
        faces.extend_from_slice(&face1);
        faces.extend_from_slice(&face2);
        if face3 != face1 {
            faces.extend_from_slice(&face3);
        }
        if face4 != face2 {
            faces.extend_from_slice(&face4);
        }

        let max_width = faces
            .iter()
            .map(|e| std::cmp::max(e.0 + 1, e.1 + 1))
            .max()
            .unwrap();

        let fig = Figure {
            figure_name: name.to_owned(),
            num_faces: (faces.len() / blocks_per_face as usize) as u8,
            faces,
            blocks_per_face,
            max_face_width: max_width,
        };

        println!(
            "Built figure {} with {} faces (blocks per face: {}, max width: {})",
            fig.name(),
            fig.num_faces(),
            fig.blocks_per_face,
            fig.max_face_width
        );
        fig
    }
    pub fn max_width(&self) -> u8 {
        self.max_face_width
    }

    pub fn name(&self) -> &String {
        &self.figure_name
    }
    pub fn num_faces(&self) -> u8 {
        self.num_faces
    }

    pub fn get_face(&self, face_index: usize) -> &[(u8, u8, u8)] {
        let face_index = face_index % self.num_faces as usize;
        let start_index = face_index * self.blocks_per_face as usize;
        &self.faces[start_index..start_index + self.blocks_per_face as usize]
    }

    pub fn iter_faces(&self) -> impl Iterator<Item = &[(u8, u8, u8)]> {
        self.faces.chunks(self.blocks_per_face as usize)
    }

    //
    // Place figure in playfield
    //
    pub fn place(&self, pf: &mut Playfield, pos: &PosDir) {
        pf.place(pos.get_pos(), self.get_face(pos.get_dir() as usize));
    }
    //
    // Remove figure from playfield
    //
    pub fn remove(&self, pf: &mut Playfield, pos: &PosDir) {
        pf.remove(pos.get_pos(), self.get_face(pos.get_dir() as usize));
    }

    pub fn test_collision(&self, pf: &Playfield, pos: &PosDir) -> bool {
        pf.test_collision(pos.get_pos(), self.get_face(pos.get_dir() as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;

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
        assert_eq!(fig.num_faces(), 4);
        assert_eq!(
            fig.get_face(0),
            &[(0, 1, 1), (1, 1, 1), (2, 1, 1), (1, 2, 1)]
        );
        assert_eq!(
            fig.get_face(1),
            &[(1, 0, 1), (1, 1, 1), (2, 1, 1), (1, 2, 1)]
        );
        assert_eq!(
            fig.get_face(2),
            &[(1, 0, 1), (0, 1, 1), (1, 1, 1), (2, 1, 1)]
        );
        assert_eq!(
            fig.get_face(3),
            &[(1, 0, 1), (0, 1, 1), (1, 1, 1), (1, 2, 1)]
        );
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
        assert_eq!(fig.num_faces(), 2);
        assert_eq!(
            fig.get_face(0),
            [(1, 0, 1), (1, 1, 1), (1, 2, 1), (1, 3, 1)]
        );
        assert_eq!(
            fig.get_face(1),
            &[(0, 1, 1), (1, 1, 1), (2, 1, 1), (3, 1, 1)]
        );
    }
    #[test]
    fn test_figure3() {
        let fig = Figure::new_from_face(
            "Figure 3",
            &[&[bl!(1), bl!(0)], &[bl!(1), bl!(1)], &[bl!(0), bl!(1)]],
        );
        assert_eq!(fig.num_faces(), 2);
        assert_eq!(
            fig.get_face(0),
            &[(0, 0, 1), (0, 1, 1), (1, 1, 1), (1, 2, 1)]
        );
        assert_eq!(
            fig.get_face(1),
            &[(0, 0, 1), (1, 0, 1), (1, 1, 1), (2, 1, 1)]
        );
    }
}
