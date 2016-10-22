#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix2D<T> {
    width: usize,
    height: usize,
    items: Vec<T>,
}

impl<T> Matrix2D<T> where T: Clone {
    pub fn new(width: usize, height: usize, initial_value: T) -> Self  {
        Matrix2D {
            width: width,
            height: height,
            items: vec![initial_value; width * height],
        }
    }
    pub fn new_init(items: &[&[T]]) -> Self {
        let height = items.len();
        let width = if height == 0 { 0 } else { items[0].len() };
        let mut m = Matrix2D {
            width: width,
            height: height,
            items: vec![items[0][0].clone(); width * height],
        };
        for y in 0..height {
            for x in 0..width {
                m.set(x, y, items[y][x].clone());
            }
        }
        return m;
    }
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.items[y * self.width + x]
    }
    pub fn set(&mut self, x: usize, y: usize, item: T) {
        self.items[y * self.width + x] = item;
    }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
}
