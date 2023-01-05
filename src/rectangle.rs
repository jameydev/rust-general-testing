#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
