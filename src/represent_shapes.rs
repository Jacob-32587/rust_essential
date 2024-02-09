pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height, }
    }
    pub fn get_area(&self) -> f64 {
        self.height * self.width
    }
    pub fn scale(&mut self, scale: f64) {
        self.height *= scale;
        self.width *= scale;
    }
}

pub fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}