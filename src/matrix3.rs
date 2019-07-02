use crate::vec3::*;

#[derive(Debug, Clone)]
pub struct Matrix3<T> {
    tl: Vec3<i32>,
    br: Vec3<i32>,
    items: Vec<T>,
}

impl<T> Matrix3<T>
where
    T: Clone,
{
    pub fn new_coords(tl: Vec3<i32>, br: Vec3<i32>, initial_value: T) -> Self {
        Matrix3 {
            items: vec![initial_value; ((br.x - tl.x) * (br.y - tl.y) * (br.z - tl.z)) as usize],
            tl,
            br,
        }
    }
    pub fn width(&self) -> u32 {
        (self.br.x - self.tl.x) as u32
    }
    pub fn height(&self) -> u32 {
        (self.br.y - self.tl.y) as u32
    }
    #[allow(dead_code)]
    pub fn depth(&self) -> u32 {
        (self.br.z - self.tl.z) as u32
    }
    #[allow(dead_code)]
    pub fn contains(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.tl.x
            && x < self.br.x
            && y >= self.tl.y
            && y < self.br.y
            && z >= self.tl.z
            && z < self.br.z
    }
    fn index_from_point(&self, point: Vec3<i32>) -> usize {
        let x = (point.x - self.tl.x) as u32;
        let y = (point.y - self.tl.y) as u32;
        let z = (point.z - self.tl.z) as u32;
        (z * self.height() * self.width() + y * self.width() + x) as usize
    }
    pub fn get(&self, point: Vec3<i32>) -> &T {
        let index = self.index_from_point(point);
        &self.items[index]
    }
    pub fn set(&mut self, point: Vec3<i32>, item: T) {
        let index = self.index_from_point(point);
        self.items[index] = item;
    }
}
