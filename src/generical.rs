use std::fmt::Display;
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

pub struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &U {
        &self.y
    }
}

pub struct _Point<T> {
    x: T,
    y: T
}

impl<T> _Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

impl _Point<f32> {
    pub fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn get_to_the_point() {
    let integer = Point { x: 5, y: 10 };
    let float = _Point { x: 1.2, y: 1.3 };
    println!("integer x = {}", integer.x());
    println!("integer y = {}", integer.y());
    println!("float x = {}", float.x());
    println!("float y = {}", float.y());
}

pub struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x , y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        }
        else {
            println!("The largest member is y = {}", self.y);
        }
    }
}