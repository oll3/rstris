use vec3::*;

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
    pub fn new(width: u32, height: u32, depth: u32, initial_value: T) -> Self {
        Self::new_coords(
            Vec3::new(0 as i32, 0 as i32, 0 as i32),
            Vec3::new(width as i32, height as i32, depth as i32),
            initial_value,
        )
    }
    pub fn new_coords(tl: Vec3<i32>, br: Vec3<i32>, initial_value: T) -> Self {
        Matrix3 {
            items: vec![initial_value; ((br.x - tl.x) * (br.y - tl.y) * (br.z - tl.z)) as usize],
            tl: tl,
            br: br,
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
    pub fn get(&self, x: i32, y: i32, z: i32) -> &T {
        let x = (x - self.tl.x) as u32;
        let y = (y - self.tl.y) as u32;
        let z = (z - self.tl.z) as u32;
        &self.items[(z * self.height() * self.width() + y * self.width() + x) as usize]
    }
    pub fn v_get(&self, v: &Vec3<i32>) -> &T {
        self.get(v.x, v.y, v.z)
    }
    pub fn tv_get(&self, tv: &ToVec3<i32>) -> &T {
        self.v_get(&tv.to_vec3())
    }
    pub fn set(&mut self, x: i32, y: i32, z: i32, item: T) {
        let x = (x - self.tl.x) as u32;
        let y = (y - self.tl.y) as u32;
        let z = (z - self.tl.z) as u32;
        let w = self.width();
        let h = self.height();
        self.items[(z * h * w + y * w + x) as usize] = item;
    }
    pub fn v_set(&mut self, v: &Vec3<i32>, item: T) {
        self.set(v.x, v.y, v.z, item);
    }
    pub fn tv_set(&mut self, tv: &ToVec3<i32>, item: T) {
        self.v_set(&tv.to_vec3(), item)
    }
}
