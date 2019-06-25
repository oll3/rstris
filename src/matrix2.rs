use crate::vec2::Vec2;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Matrix2<T> {
    w: u32,
    h: u32,
    items: Vec<T>,
}

impl<T> Matrix2<T>
where
    T: Clone,
{
    pub fn from_size(width: u32, height: u32, initial_value: T) -> Matrix2<T> {
        Matrix2 {
            items: vec![initial_value; (height * width) as usize],
            w: width,
            h: height,
        }
    }

    pub fn from_items(items: &[&[T]]) -> Self {
        let height = items.len() as u32;
        let width = if height == 0 { 0 } else { items[0].len() } as u32;
        let mut matrix = Self::from_size(width, height, items[0][0].clone());
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                matrix.set(Vec2 { x, y }, items[y as usize][x as usize].clone());
            }
        }
        matrix
    }

    pub fn clone_from_slice(&mut self, other: &Matrix2<T>) {
        self.items.clone_from_slice(&other.items);
    }

    pub fn row_iter(&self) -> impl Iterator<Item = &[T]> {
        self.items.chunks(self.w as usize)
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn contains(&self, point: Vec2<i32>) -> bool {
        point.x >= 0
            && point.x < self.width() as i32
            && point.y >= 0
            && point.y < self.height() as i32
    }

    fn index_from_point(&self, point: Vec2<i32>) -> usize {
        let p0 = point.x as usize;
        let p1 = point.y as usize;
        (p1 * self.width() as usize + p0)
    }

    pub fn get(&self, point: Vec2<i32>) -> &T {
        &self.items[self.index_from_point(point)]
    }

    pub fn set(&mut self, point: Vec2<i32>, item: T) {
        let index = self.index_from_point(point);
        self.items[index] = item;
    }

    // Merge another matrix with self
    pub fn merge<F>(&mut self, at: Vec2<i32>, other: &Matrix2<T>, mut merge_func: F)
    where
        F: FnMut(&mut T, &T),
    {
        let mut other_x = 0;
        let mut other_y = 0;
        for other_item in other.items.iter() {
            let point = Vec2 {
                x: at.x + other_x,
                y: at.y + other_y,
            };
            if self.contains(point) {
                let index = self.index_from_point(point);
                merge_func(&mut self.items[index], other_item);
            }
            other_x += 1;
            if other_x >= other.width() as i32 {
                other_x = 0;
                other_y += 1;
            }
        }
    }
    // Test if another matrix overlaps with self
    pub fn test_overlap<F>(&self, at: Vec2<i32>, other: &Matrix2<T>, mut test_func: F) -> bool
    where
        F: FnMut(Option<&T>, &T) -> bool,
    {
        let mut other_x = 0;
        let mut other_y = 0;
        for other_item in other.items.iter() {
            let point = Vec2 {
                x: at.x + other_x,
                y: at.y + other_y,
            };

            let item = if self.contains(point) {
                Some(&self.items[self.index_from_point(point)])
            } else {
                None
            };

            if test_func(item, other_item) {
                return true;
            }
            other_x += 1;
            if other_x >= other.width() as i32 {
                other_x = 0;
                other_y += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_same_size() {
        let mut m1 = Matrix2::from_items(&[
            &[false, false, false],
            &[false, true, false],
            &[false, true, false],
            &[false, false, false],
        ]);
        let m2 = Matrix2::from_items(&[
            &[false, true, false],
            &[false, false, false],
            &[false, false, false],
            &[false, true, false],
        ]);
        let expected = Matrix2::from_items(&[
            &[false, true, false],
            &[false, true, false],
            &[false, true, false],
            &[false, true, false],
        ]);

        m1.merge(Vec2 { x: 0, y: 0 }, &m2, |a, b| {
            if *b {
                *a = *b;
            }
        });

        assert_eq!(m1, expected);
    }
    #[test]
    fn test_merge_different_size() {
        let mut m1 = Matrix2::from_items(&[
            &[false, false, false],
            &[false, true, false],
            &[false, true, false],
            &[true, true, false],
        ]);
        let m2 = Matrix2::from_items(&[&[false, true], &[false, false], &[true, false]]);
        let expected = Matrix2::from_items(&[
            &[false, false, false],
            &[false, true, true],
            &[false, true, false],
            &[true, true, false],
        ]);

        m1.merge(Vec2 { x: 1, y: 1 }, &m2, |a, b| {
            if *b {
                *a = *b;
            }
        });

        assert_eq!(m1, expected);
    }
    #[test]
    fn test_merge_outside() {
        let mut m1 = Matrix2::from_items(&[
            &[false, false, false],
            &[false, true, false],
            &[false, true, false],
            &[true, true, false],
        ]);
        let m2 = Matrix2::from_items(&[
            &[false, true, false],
            &[false, false, false],
            &[false, false, false],
            &[false, true, false],
        ]);
        let expected = Matrix2::from_items(&[
            &[false, false, false],
            &[false, true, false],
            &[true, true, false],
            &[true, true, false],
        ]);

        m1.merge(Vec2 { x: -1, y: -1 }, &m2, |a, b| {
            if *b {
                *a = *b;
            }
        });

        assert_eq!(m1, expected);
    }
    #[test]
    fn test_overlap() {
        let m1 = Matrix2::from_items(&[
            &[false, false, false],
            &[false, true, false],
            &[false, true, false],
            &[true, false, false],
        ]);

        // Won't overlap m1
        let no_overlap = Matrix2::from_items(&[
            &[false, true, false],
            &[false, false, false],
            &[false, false, false],
            &[false, true, false],
        ]);
        assert_eq!(
            m1.test_overlap(Vec2 { x: 0, y: 0 }, &no_overlap, |a, b| match a {
                Some(ai) => *ai == true && *b == true,
                None => *b == true,
            }),
            false
        );

        // Will overlap m1
        let overlap = Matrix2::from_items(&[
            &[false, true, false],
            &[false, true, false],
            &[false, true, false],
        ]);
        assert_eq!(
            m1.test_overlap(Vec2 { x: 0, y: 0 }, &overlap, |a, b| match a {
                Some(ai) => *ai == true && *b == true,
                None => *b == true,
            }),
            true
        );
    }
}
