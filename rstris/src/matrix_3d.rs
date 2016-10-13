#[derive(Debug, Clone)]
pub struct Matrix3D<T> {
    width: usize,
    height: usize,
    depth: usize,
    items: Vec<T>,
}

impl<T> Matrix3D<T> where T: Clone {
    pub fn new(width: usize, height: usize, depth: usize,
               initial_value: T) -> Self  {
        Matrix3D {
            width: width,
            height: height,
            depth: depth,
            items: vec![initial_value; width * height * depth],
        }
    }
    pub fn get(&self, x: usize, y: usize, z: usize) -> &T {
        &self.items[z * self.height * self.width + y * self.width + x]
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, item: T) {
        self.items[z * self.height * self.width + y * self.width + x] = item;
    }
//    pub fn width(&self) -> usize { self.width }
//    pub fn height(&self) -> usize { self.height }
//    pub fn depth(&self) -> usize { self.depth }
}
