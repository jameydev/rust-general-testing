#![allow(unused)]
extern crate core;

use crate::aggregator::{Summary, Toot};
use crate::generical::get_to_the_point;
use error_thing::*;
use fizzbuzz::*;
use garden::vegetables::Asparagus;
use life::*;
use median::*;
use rectangle::Rectangle;
use std::collections::HashMap;
use std::io;
use stuff::*;
use tshirt::*;
use user::User;

mod aggregator;
mod error_thing;
mod fizzbuzz;
mod garden;
mod generical;
mod life;
mod median;
mod rectangle;
mod stuff;
mod tshirt;
mod user;

pub mod general_testing {
    use super::*;
    pub fn old_crapp() {
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

    pub fn current_bs() {
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

    pub fn user_input() {
        println!("\nWhat's your name?");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Didn't receive input");
        let greeting = String::from("Nice to meet you");
        println!("{}, {}!", &greeting, name.trim_end());
    }

    pub fn its_lifetimes_jim() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("\nThe longest string is {}", result);

        some_text();
        println!();

        let the_longo = longest_with_an_announcement("ballin", "shot callin", "Yeet!");
        println!("Longest string = {}", the_longo);
    }

    pub fn tshirt_debuggo() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Orange, ShirtColor::Green],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }
}

fn main() {
    use general_testing::*;
    old_crapp();
    current_bs();
    user_input();
    its_lifetimes_jim();
    println!();
    tshirt_debuggo();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lrgr_rect_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_contain(&smaller));
    }

    #[test]
    fn smaller_rect_cannot_hold_lrger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_contain(&larger));
    }

    #[test]
    #[should_panic]
    fn guess_greater_than_100_panics() {
        Guess::new(200);
    }

    #[test]
    fn tee_shirt_color_same_if_stocked() {
        let store = tshirt::Inventory {
            shirts: vec![
                ShirtColor::Blue,
                ShirtColor::Red,
                ShirtColor::Orange,
                ShirtColor::Blue,
            ],
        };

        let user_pref = Some(ShirtColor::Red);
        let give_it_away = store.giveaway(user_pref);

        assert_eq!(give_it_away, ShirtColor::Red);
    }

    #[test]
    fn tee_shirt_color_most_if_no_pref() {
        let store = tshirt::Inventory {
            shirts: vec![
                ShirtColor::Blue,
                ShirtColor::Red,
                ShirtColor::Green,
                ShirtColor::Red,
            ],
        };

        let user_pref = None;
        let give_it_away = store.giveaway(user_pref);

        assert_eq!(give_it_away, ShirtColor::Red);
    }
}
