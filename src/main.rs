#![allow(unused)]
extern crate core;

use std::collections::HashMap;
use std::io;
use error_thing::*;
use fizzbuzz::*;
use garden::vegetables::Asparagus;
use median::*;
use rectangle::Rectangle;
use stuff::*;
use user::User;
use crate::aggregator::{Summary, Toot};
use crate::generical::get_to_the_point;
use life::*;

mod garden;
mod stuff;
mod user;
mod rectangle;
mod median;
mod fizzbuzz;
mod error_thing;
mod generical;
mod aggregator;
mod life;

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

    println!();

    let toot = Toot {
        username: String::from("jameydev"),
        mastodon_instance: String::from("hachyderm.io"),
        content: String::from("I nooted my profile image. NOOT NOOT FOR TOOTS😂"),
        is_reply: false,
        is_reblog: false,
    };

    println!("1 new toot: {}", toot.summarize());
}

fn user_input() {
    println!("\nWhat's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Didn't receive input");
    let greeting = String::from("Nice to meet you");
    println!("{}, {}!", &greeting, name.trim_end());
}

fn main() {
    old_crapp();
    current_bs();
    user_input();

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("\nThe longest string is {}", result);

    some_text();
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