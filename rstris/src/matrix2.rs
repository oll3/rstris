use vec2::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Matrix2<T> {
    tl: Vec2<i32>,
    br: Vec2<i32>,
    items: Vec<T>,
}

impl<T> Matrix2<T> where T: Clone {
    pub fn new(width: u32, height: u32, initial_value: T) -> Self  {
        Self::new_coords(Vec2::new(0 as i32, 0 as i32),
                         Vec2::new(width as i32, height as i32),
                         initial_value)
    }
    pub fn new_coords(tl: Vec2<i32>, br: Vec2<i32>,
                      initial_value: T) -> Self  {
        Matrix2 {
            items: vec![initial_value;
                        ((br.x - tl.x) *
                        (br.y - tl.y)) as usize],
            tl: tl,
            br: br,
        }
    }
    pub fn new_init(items: &[&[T]]) -> Self {
        let height = items.len() as u32;
        let width = if height == 0 { 0 } else { items[0].len() as u32 };
        let mut m = Self::new(width, height, items[0][0].clone());
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                m.set(x, y, items[y as usize][x as usize].clone());
            }
        }
        return m;
    }
    pub fn width(&self) -> u32 { (self.br.x - self.tl.x) as u32 }
    pub fn height(&self) -> u32 { (self.br.y - self.tl.y) as u32 }
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.tl.x && x < self.br.x &&
            y >= self.tl.y && y < self.br.y
    }
    pub fn get(&self, x: i32, y: i32) -> &T {
        let x = (x - self.tl.x) as u32;
        let y = (y - self.tl.y) as u32;
        &self.items[(y * self.width() +
                     x) as usize]
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
        let w = self.width();
        self.items[(y * w + x) as usize] = item;
    }
    pub fn v_set(&mut self, v: &Vec2<i32>, item: T) {
        self.set(v.x, v.y, item);
    }
    pub fn tv_set(&mut self, tv: &ToVec2<i32>, item: T) {
        self.v_set(&tv.to_vec2(), item)
    }
}
