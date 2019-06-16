use crate::vec2::Vec2;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Matrix2<T> {
    w: u32,
    h: u32,
    items: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub struct ItemPoint<'a, T: 'a> {
    pub point: Vec2<i32>,
    pub item: &'a T,
}

pub struct ItemIt<'a, T: 'a>
where
    T: Clone,
{
    pub point: Vec2<i32>,
    matrix: &'a Matrix2<T>,
}

impl<'a, T: 'a> Iterator for ItemIt<'a, T>
where
    T: Clone,
{
    type Item = ItemPoint<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.x as u32 >= self.matrix.width() {
            self.point.x = 0;
            self.point.y += 1;
        }
        if self.point.y as u32 >= self.matrix.height() {
            return None;
        }
        let point = ItemPoint {
            point: self.point,
            item: self.matrix.get(self.point),
        };
        self.point.x += 1;
        Some(point)
    }
}

#[derive(Debug, PartialEq)]
pub struct RowPoint<'a, T: 'a> {
    pub point: i32,
    pub items: &'a [T],
}

pub struct RowIt<'a, T: 'a>
where
    T: Clone,
{
    pub point: i32,
    matrix: &'a Matrix2<T>,
}

impl<'a, T: 'a> Iterator for RowIt<'a, T>
where
    T: Clone,
{
    type Item = RowPoint<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.point as u32 >= self.matrix.height() {
            return None;
        }
        let width = self.matrix.width() as usize;
        let index = self.matrix.index_from_point((0, self.point).into());
        let point = RowPoint {
            point: self.point,
            items: &self.matrix.items[index..index + width],
        };
        self.point += 1;
        Some(point)
    }
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

    pub fn iter(&self) -> ItemIt<T> {
        ItemIt {
            point: Vec2 { x: 0, y: 0 },
            matrix: &self,
        }
    }

    pub fn row_iter(&self) -> RowIt<T> {
        RowIt {
            point: 0,
            matrix: &self,
        }
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
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
        other.items.iter().enumerate().for_each(|(other_index, b)| {
            let other_index = other_index as i32;
            let y = other_index / (other.width() as i32);
            let x = other_index - y * (other.width() as i32);
            let point = Vec2 {
                x: at.x + x,
                y: at.y + y,
            };
            if self.contains(point) {
                let index = self.index_from_point(point);
                merge_func(&mut self.items[index], b);
            }
        });
    }
    // Test if another matrix overlaps with self
    pub fn test_overlap<F>(&self, at: Vec2<i32>, other: &Matrix2<T>, mut test_func: F) -> bool
    where
        F: FnMut(Option<&T>, &T) -> bool,
    {
        for (other_index, other_item) in other.items.iter().enumerate() {
            let other_index = other_index as i32;
            let y = other_index / (other.width() as i32);
            let x = other_index - y * (other.width() as i32);
            let point = Vec2 {
                x: at.x + x,
                y: at.y + y,
            };
            let item = if self.contains(point) {
                let index = self.index_from_point(point);
                Some(&self.items[index])
            } else {
                None
            };

            if test_func(item, other_item) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut m1 = Matrix2::from_size(10, 10, false);
        m1.set((2, 3).into(), true);
        m1.set((9, 9).into(), true);
        let r: Vec<ItemPoint<bool>> = m1.iter().filter(|p| *p.item == true).collect();
        assert_eq!(
            r[0],
            ItemPoint {
                point: Vec2 { x: 2, y: 3 },
                item: &true
            }
        );
        assert_eq!(
            r[1],
            ItemPoint {
                point: Vec2 { x: 9, y: 9 },
                item: &true
            }
        );
    }

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
