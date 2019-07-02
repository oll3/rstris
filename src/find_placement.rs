use crate::figure::Figure;
use crate::playfield::*;
use crate::position::Position;

pub fn find_placement(placements: &mut Vec<Position>, pf: &Playfield, fig: &Figure) {
    for (dir, face) in fig.iter_faces().enumerate() {
        for x in -(i32::from(fig.max_width() / 2))..pf.width() as i32 {
            let mut last_pos = None;
            for y in 0..pf.height() {
                let pos = (x as i32, y as i32).into();
                if pf.test_collision(pos, face) {
                    if let Some(p) = last_pos {
                        placements.push(p);
                    }
                    last_pos = None;
                } else {
                    last_pos = Some((x as i32, y as i32, dir as i32).into());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;
    use crate::figure::*;

    macro_rules! bl {
        ($x:expr) => {
            match $x {
                0 => Block::Clear,
                _ => Block::Set($x),
            }
        };
    }

    fn fig2() -> Figure {
        Figure::new_from_face(
            "2",
            &[
                &[bl!(0), bl!(0), bl!(0)],
                &[bl!(2), bl!(2), bl!(2)],
                &[bl!(0), bl!(0), bl!(2)],
            ],
        )
    }

    fn fig3() -> Figure {
        Figure::new_from_face(
            "3",
            &[
                &[bl!(0), bl!(0), bl!(3)],
                &[bl!(3), bl!(3), bl!(3)],
                &[bl!(0), bl!(0), bl!(0)],
            ],
        )
    }

    #[test]
    fn find1() {
        let mut placements = Vec::new();
        let fig = fig2();
        let pf = Playfield::new("pf1", 10, 20);
        find_placement(&mut placements, &pf, &fig);
        assert_eq!(placements.len(), 34);
    }

    #[test]
    fn find2() {
        let mut placements = Vec::new();
        let fig = fig3();
        let pf = Playfield::new("pf1", 20, 20);
        find_placement(&mut placements, &pf, &fig);
        assert_eq!(placements.len(), 74);
    }
    /*
    use test::Bencher;
    #[bench]
    fn find_fig2_10x20(b: &mut Bencher) {
        let start_pos = fig2(0, 0, 0);
        let pf = Playfield::new("pf1", 10, 20);
        b.iter(|| {
            let placings = find_placement_quick(&pf, &start_pos);
        });
    }
    #[bench]
    fn find_fig3_20x20(b: &mut Bencher) {
        let start_pos = fig3(0, 0, 0);
        let pf = Playfield::new("pf1", 20, 20);
        b.iter(|| {
            let placings = find_placement_quick(&pf, &start_pos);
        });
    }*/
}
