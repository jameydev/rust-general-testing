extern crate core;

use std::collections::HashMap;

pub use fizzbuzz::*;
use garden::vegetables::Asparagus;
pub use median::*;
pub use rectangle::Rectangle;
pub use stuff::*;
pub use user::User;
pub use error_thing::*;

pub mod garden;
pub mod stuff;
pub mod user;
pub mod rectangle;
mod median;
mod fizzbuzz;
mod error_thing;
mod generical;

fn main() {
    stringy_thingy();
    println!();

    user_thing();
    println!();

    rectangular_thing();
    println!();

    ip_thing();
    println!();

    one_more_thing();
    println!();

    coin_thing();
    println!();

    garden_thing();
    println!();

    vector_thingy();
    println!();

    mapped_hashes_hash_mapping();
    println!();

    // Lol, this doesn't work
    // median_tester();
    // println!();
    mode_tester();
    println!();

    fizz_buzz(100);
    println!();

    get_file();
    println!();

    let money = last_char_of_first_ln("show me that money, fam");
    println!("The money is: {}", money.unwrap());
}

// lame way
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// better way with tuples
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Better version before struct w/ area method
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }