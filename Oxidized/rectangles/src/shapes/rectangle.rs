#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.height * self.width
    }
    pub fn can_hold(&self, other_rec: &Rectangle) -> bool {
        self.area() > other_rec.area()
    }
}