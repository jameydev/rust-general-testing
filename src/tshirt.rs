#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
    Orange,
    Green
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn largest(shirt_count_vec: &Vec<usize>) -> usize {
        let mut maximum = 0;
        for shirt_count in shirt_count_vec {
            if *shirt_count > maximum {
                maximum = *shirt_count;
            }
        }

        maximum
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_orange = 0;
        let mut num_green = 0;
        
        let mut shirt_count_vec = Vec::new();
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Orange => num_orange += 1,
                ShirtColor::Green => num_green += 1
            }
        }

        shirt_count_vec.push(num_red);
        shirt_count_vec.push(num_blue);
        shirt_count_vec.push(num_orange);
        shirt_count_vec.push(num_green);

        for item in &shirt_count_vec {
            println!("{item}");
        }

        if Self::largest(&shirt_count_vec) == num_red {
            ShirtColor::Red
        }
        else if Self::largest(&shirt_count_vec) == num_blue {
            ShirtColor::Blue
        }
        else if Self::largest(&shirt_count_vec) == num_orange {
            ShirtColor::Orange
        }
        else {
            ShirtColor::Green
        }
        
    }
}

