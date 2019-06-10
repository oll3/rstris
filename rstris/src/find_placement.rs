extern crate time;

use crate::figure_pos::*;
use crate::matrix3::*;
use crate::movement::*;
use crate::playfield::*;
use crate::pos_dir::*;
use crate::vec3::Vec3;
use std::collections::LinkedList;

static DEBUG_FIND_PLACEMENT: bool = false;

pub fn find_placement_quick(pf: &Playfield, fig_pos: &FigurePos) -> Vec<PosDir> {
    let current_ticks = time::precise_time_ns();
    let mut placements: Vec<PosDir> = Vec::new();
    let start_pos = fig_pos.get_position();
    let fig = fig_pos.get_figure();

    if DEBUG_FIND_PLACEMENT {
        println!(
            "Find valid placements for figure {} (starting at {:?})",
            fig.get_name(),
            start_pos
        );
    }

    for dir in 0..fig.faces().len() {
        let fig_face_width = fig.faces()[dir].width() as i32;
        for x in -fig_face_width..pf.width() as i32 {
            let mut last_pos = Some(PosDir::new((x as i32, 0, dir as i32)));
            for y in 1..pf.height() {
                let tmp_pos = PosDir::new((x as i32, y as i32, dir as i32));
                if fig.test_collision(&pf, &tmp_pos) {
                    if let Some(p) = last_pos {
                        placements.push(p);
                    }
                    last_pos = None;
                } else {
                    last_pos = Some(tmp_pos);
                }
            }
        }
    }

    if DEBUG_FIND_PLACEMENT {
        println!(
            "Found {} valid placements for {} ({} ms)",
            placements.len(),
            fig.get_name(),
            (time::precise_time_ns() - current_ticks) as f64 / 1_000_000.0
        );
    }

    placements
}

pub fn find_placement(pf: &Playfield, fig_pos: &FigurePos) -> Vec<PosDir> {
    let current_ticks = time::precise_time_ns();
    let mut placements: Vec<PosDir> = Vec::new();
    let mut moves: LinkedList<PosDir> = LinkedList::new();
    let mut visited: Matrix3<bool> = Matrix3::new(pf.width() as u32, pf.height() as u32, 4, false);
    let mut it_cnt = 0;
    let start_pos = fig_pos.get_position();
    let fig = fig_pos.get_figure();

    if DEBUG_FIND_PLACEMENT {
        println!(
            "Find valid placements for figure {} (starting at {:?})",
            fig.get_name(),
            start_pos
        );
    }

    if fig.test_collision(pf, &start_pos) {
        println!(
            "Invalid starting point ({:?}) for figure {}",
            start_pos,
            fig.get_name()
        );
        return placements;
    }

    visited.set(*start_pos, true);
    moves.push_back(*start_pos);

    while !moves.is_empty() {
        let current_pos = moves.pop_front().unwrap();

        // Visist all the closest positions that has not been visited
        // already (one left, right, down, rotate cw).
        let tmp_pos = PosDir::apply_move(&current_pos, &Movement::MoveLeft);
        let point: Vec3<i32> = tmp_pos;
        if !visited.get(point) && !fig.test_collision(&pf, &tmp_pos) {
            visited.set(point, true);
            moves.push_back(tmp_pos);
        }
        let tmp_pos = PosDir::apply_move(&current_pos, &Movement::MoveRight);
        if !visited.get(tmp_pos) && !fig.test_collision(&pf, &tmp_pos) {
            visited.set(tmp_pos, true);
            moves.push_back(tmp_pos);
        }
        let tmp_pos = PosDir::apply_move(&current_pos, &Movement::RotateCW);
        if tmp_pos.get_dir() < fig.faces().len() as i32
            && !visited.get(tmp_pos)
            && !fig.test_collision(&pf, &tmp_pos)
        {
            visited.set(tmp_pos, true);
            moves.push_back(tmp_pos);
        }

        // Down is special. If we can't move down from current position then
        // the current position is a valid placement.
        let tmp_pos = PosDir::apply_move(&current_pos, &Movement::MoveDown);
        if fig.test_collision(&pf, &tmp_pos) {
            // Valid placement
            // println!("Valid position: {:?}", tmp_pos);
            placements.push(current_pos);
        } else if !visited.get(tmp_pos) {
            moves.push_back(tmp_pos);
            visited.set(tmp_pos, true);
        }
        it_cnt += 1;
    }

    if DEBUG_FIND_PLACEMENT {
        println!(
            "Found {} valid placements for {} (iterated {} times, {} ms)",
            placements.len(),
            fig.get_name(),
            it_cnt,
            (time::precise_time_ns() - current_ticks) as f64 / 1_000_000.0
        );
    }

    placements
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

    fn fig2(x: i32, y: i32, dir: i32) -> FigurePos {
        FigurePos::new(
            Figure::new_from_face(
                "2",
                &[
                    &[bl!(0), bl!(0), bl!(0)],
                    &[bl!(2), bl!(2), bl!(2)],
                    &[bl!(0), bl!(0), bl!(2)],
                ],
            ),
            PosDir::new((x, y, dir)),
        )
    }

    fn fig3(x: i32, y: i32, dir: i32) -> FigurePos {
        FigurePos::new(
            Figure::new_from_face(
                "3",
                &[
                    &[bl!(0), bl!(0), bl!(3)],
                    &[bl!(3), bl!(3), bl!(3)],
                    &[bl!(0), bl!(0), bl!(0)],
                ],
            ),
            PosDir::new((x, y, dir)),
        )
    }

    #[test]
    fn find1() {
        let start_pos = fig2(0, 0, 0);
        let pf = Playfield::new("pf1", 10, 20);
        let placings = find_placement_quick(&pf, &start_pos);
        assert_eq!(placings.len(), 52);
    }

    #[test]
    fn find2() {
        let start_pos = fig3(0, 0, 0);
        let pf = Playfield::new("pf1", 20, 20);
        let placings = find_placement_quick(&pf, &start_pos);
        assert_eq!(placings.len(), 92);
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
