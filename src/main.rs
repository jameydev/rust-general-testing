extern crate core;

use std::collections::HashMap;

use error_thing::*;
use fizzbuzz::*;
use garden::vegetables::Asparagus;
use median::*;
use rectangle::Rectangle;
use stuff::*;
use user::User;
use crate::generical::get_to_the_point;

mod garden;
mod stuff;
mod user;
mod rectangle;
mod median;
mod fizzbuzz;
mod error_thing;
mod generical;

fn old_crapp() {
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
}

fn current_bs() {
    let money = last_char_of_first_ln("show me that money, fam");
    println!("The money is: {}", money.unwrap());

    println!();
    get_to_the_point();
}

fn main() {
    old_crapp();
    current_bs();
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