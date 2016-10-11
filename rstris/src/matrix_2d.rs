#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Matrix2D<T> {
    width: usize,
    height: usize,
    items: Vec<T>,
}

impl<T> Matrix2D<T> where T: Clone {
    pub fn new(width: usize, height: usize, initial_value: T)
               -> Self  {
        Matrix2D {
            width: width,
            height: height,
            items: vec![initial_value; width * height],
        }
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
