use vec2::Vec2;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Matrix2<T> {
    tl: Vec2<i32>, // top left coordinate
    br: Vec2<i32>, // bottom rigth coordinat
    pub items: Vec<T>,
}

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
    fn next<'s>(&'s mut self) -> Option<Self::Item> {
        if self.point.x >= self.matrix.br.x {
            self.point.x = self.matrix.tl.x;
            self.point.y += 1;
        }
        if self.point.y >= self.matrix.br.y {
            return None;
        }
        let point = ItemPoint {
            point: self.point,
            item: self.matrix.get(self.point),
        };
        self.point.x += 1;
        return Some(point);
    }
}

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
    fn next<'s>(&'s mut self) -> Option<Self::Item> {
        if self.point >= self.matrix.br.y {
            return None;
        }
        let width = self.matrix.width() as usize;
        let index = self.matrix.index_from_point((0, self.point).into());
        let point = RowPoint {
            point: self.point,
            items: &self.matrix.items[index..index + width],
        };
        self.point += 1;
        return Some(point);
    }
}

impl<T> Matrix2<T>
where
    T: Clone,
{
    pub fn from_size(width: u32, height: u32, initial_value: T) -> Matrix2<T> {
        Self::from_coords(
            Vec2::new((0, 0)),
            Vec2::new((width as i32, height as i32)),
            initial_value,
        )
    }
    pub fn from_coords(tl: Vec2<i32>, br: Vec2<i32>, initial_value: T) -> Self {
        let w = br.x - tl.x;
        let h = br.y - tl.y;
        Matrix2 {
            items: vec![initial_value; (h * w) as usize],
            tl: tl,
            br: br,
        }
    }
    pub fn from_items(items: &[&[T]]) -> Self {
        let height = items.len() as u32;
        let width = if height == 0 {
            0
        } else {
            items[0].len() as u32
        };
        let mut m = Self::from_size(width, height, items[0][0].clone());
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                m.set(Vec2 { x, y }, items[y as usize][x as usize].clone());
            }
        }
        return m;
    }

    pub fn iter<'a>(&'a self) -> ItemIt<'a, T> {
        ItemIt {
            point: self.tl,
            matrix: &self,
        }
    }

    pub fn row_iter<'a>(&'a self) -> RowIt<'a, T> {
        RowIt {
            point: self.tl.y,
            matrix: &self,
        }
    }

    pub fn width(&self) -> u32 {
        (self.br.x - self.tl.x) as u32
    }
    pub fn height(&self) -> u32 {
        (self.br.y - self.tl.y) as u32
    }
    pub fn contains(&self, point: Vec2<i32>) -> bool {
        point.x >= self.tl.x && point.x < self.br.x && point.y >= self.tl.y && point.y < self.br.y
    }
    fn index_from_point(&self, point: Vec2<i32>) -> usize {
        let p0 = (point.x - self.tl.x) as u32;
        let p1 = (point.y - self.tl.y) as u32;
        (p1 * self.width() + p0) as usize
    }
    pub fn get(&self, point: Vec2<i32>) -> &T {
        &self.items[self.index_from_point(point)]
    }
    pub fn set(&mut self, point: Vec2<i32>, item: T) {
        let index = self.index_from_point(point);
        self.items[index] = item;
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
