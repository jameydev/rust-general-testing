use std::collections::HashMap;
use crate::{Rectangle, User};
use crate::garden::vegetables::Asparagus;

#[derive(Debug)]
pub enum IpAddress {
    IPv4(u8, u8, u8, u8),
    IPv6(String)
}

// #[derive(Debug)]
// struct IPAddress {
//     kind: IpAddrKind,
//     address: String
// }

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

// impl Message {
//     pub fn call(&self) {
//
//     }
// }

// fn countdown(n: u8) {
//     for number in (1..n + 1).rev() {
//         println!("{number}...");
//     }
//
//     println!("BLAST OFF!");
// }

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Lousiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming

}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

pub fn stringy_thingy() {
    let s = String::from("mega-ultra chicken");
    let word = first_word(&s);
    println!("\nFirst word: {}", word);
}

pub fn user_thing() {
    let user = User::build_user(String::from("jesus@god.damn"), String::from("jesuschristdude"));
    dbg!(&user);
}

pub fn rectangular_thing() {
    let rect = Rectangle {
        width: 25,
        height: 30,
    };

    dbg!(&rect);
    println!("The area of rect is {}px", rect.area());

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 35,
        height: 50,
    };

    if rect.can_contain(&rect2) {
        println!("rect can hold rect2");
    }
    else {
        println!("Lol, nope!");
    }

    if rect.can_contain(&rect3) {
        println!("rect can hold rect3");
    }
    else {
        println!("Lol, nope!");
    }
}

pub fn ip_thing() {
    let home = IpAddress::IPv4(127, 0, 0, 1);
    let loopback = IpAddress::IPv6(String::from("::1"));
    dbg!(home);
    dbg!(loopback);
}

pub fn coin_thing() {
    let quarter = Coin::Quarter(UsState::Texas);
    let qtr_value = value_in_cents(quarter);

    println!("Quarters are worth {} cents\n", qtr_value);

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}.\n", state)
    }
    else {
        count += 1;
    }
}

pub fn one_more_thing() {
    let x = 32;
    println!("x = {:?}\n", plus_one(Some(x)));
}

pub fn garden_thing() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!\n", plant);
}

pub fn vector_thingy() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    for i in 6..10 {
        v.push(i);
    }

    for j in &v {
        print!("{j} ");
    }

    let third = &v[2];
    println!("\nThe third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }
}

pub fn mapped_hashes_hash_mapping() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Scores for the Hackinator PenTesting Tournament:");
    for (key, value) in &scores {
        println!("{key} Team: {value} points");
    }

    let txt = "hello my darkness, hello my honey, hello my oldtime friend";
    let mut map = HashMap::new();

    for word in txt.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
}