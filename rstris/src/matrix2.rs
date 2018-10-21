use std::slice::Iter;
use vec2::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Matrix2<T> {
    tl: Vec2<i32>, // top left coordinate
    br: Vec2<i32>, // bottom rigth coordinat
    pub items: Vec<Vec<T>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct MatrixPoint<'a, T: 'a> {
    pub x: i32,
    pub y: i32,
    pub item: &'a T,
}

pub struct MatrixIt<'a, T: 'a> {
    x: usize,
    y: usize,
    matrix: &'a Matrix2<T>,
}

impl<'a, T: 'a> Iterator for MatrixIt<'a, T> {
    type Item = MatrixPoint<'a, T>;
    fn next<'s>(&'s mut self) -> Option<Self::Item> {
        if self.x >= self.matrix.items[0].len() {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.matrix.items.len() {
            return None;
        }
        let x = self.x;
        let y = self.y;
        self.x += 1;
        let point = MatrixPoint {
            x: x as i32 + self.matrix.tl.x,
            y: y as i32 + self.matrix.tl.y,
            item: &self.matrix.items[y][x],
        };
        return Some(point);
    }
}

impl<T> Matrix2<T>
where
    T: Clone,
{
    pub fn new(width: u32, height: u32, initial_value: T) -> Matrix2<T> {
        Self::new_coords(
            Vec2::new(0 as i32, 0 as i32),
            Vec2::new(width as i32, height as i32),
            initial_value,
        )
    }

    pub fn iter<'a>(&'a self) -> MatrixIt<'a, T> {
        MatrixIt {
            y: 0,
            x: 0,
            matrix: &self,
        }
    }

    pub fn line_iter<'a>(&'a self) -> Iter<'a, Vec<T>> {
        self.items.iter()
    }

    pub fn new_coords(tl: Vec2<i32>, br: Vec2<i32>, initial_value: T) -> Self {
        let w = br.x - tl.x;
        let h = br.y - tl.y;
        Matrix2 {
            items: vec![vec![initial_value; w as usize]; h as usize],
            tl: tl,
            br: br,
        }
    }
    pub fn new_init(items: &[&[T]]) -> Self {
        let height = items.len() as u32;
        let width = if height == 0 {
            0
        } else {
            items[0].len() as u32
        };
        let mut m = Self::new(width, height, items[0][0].clone());
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                m.set(x, y, items[y as usize][x as usize].clone());
            }
        }
        return m;
    }
    pub fn width(&self) -> u32 {
        (self.br.x - self.tl.x) as u32
    }
    pub fn height(&self) -> u32 {
        (self.br.y - self.tl.y) as u32
    }
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.tl.x && x < self.br.x && y >= self.tl.y && y < self.br.y
    }
    pub fn get(&self, x: i32, y: i32) -> &T {
        let x = (x - self.tl.x) as u32;
        let y = (y - self.tl.y) as u32;
        &self.items[y as usize][x as usize]
    }
    pub fn v_get(&self, v: &Vec2<i32>) -> &T {
        self.get(v.x, v.y)
    }
    pub fn tv_get(&self, tv: &ToVec2<i32>) -> &T {
        self.v_get(&tv.to_vec2())
    }
    pub fn set(&mut self, x: i32, y: i32, item: T) {
        let x = (x - self.tl.x) as u32;
        let y = (y - self.tl.y) as u32;
        self.items[y as usize][x as usize] = item;
    }
    pub fn v_set(&mut self, v: &Vec2<i32>, item: T) {
        self.set(v.x, v.y, item);
    }
    pub fn tv_set(&mut self, tv: &ToVec2<i32>, item: T) {
        self.v_set(&tv.to_vec2(), item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matrix2::*;

    #[test]
    fn test_iter() {
        let mut m1 = Matrix2::new(10, 10, false);
        m1.set(2, 3, true);
        m1.set(9, 9, true);
        let r: Vec<MatrixPoint<bool>> = m1.iter().filter(|p| *p.item == true).collect();
        assert_eq!(
            r[0],
            MatrixPoint {
                x: 2,
                y: 3,
                item: &true
            }
        );
        assert_eq!(
            r[1],
            MatrixPoint {
                x: 9,
                y: 9,
                item: &true
            }
        );
    }
}
