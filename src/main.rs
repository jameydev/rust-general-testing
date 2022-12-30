use std::collections::HashMap;
use garden::vegetables::Asparagus;

pub mod garden;
pub mod stuff;
pub mod user;
pub mod rectangle;

pub use rectangle::Rectangle;
pub use stuff::*;
pub use user::User;

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
}

fn stringy_thingy() {
    let s = String::from("mega-ultra chicken");
    let word = stuff::first_word(&s);
    println!("\nFirst word: {}", word);
}

fn user_thing() {
    let user = User::build_user(String::from("jesus@god.damn"), String::from("jesuschristdude"));
    dbg!(&user);
}

fn rectangular_thing() {
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

fn ip_thing() {
    let home = IpAddress::IPv4(127, 0, 0, 1);
    let loopback = IpAddress::IPv6(String::from("::1"));
    dbg!(home);
    dbg!(loopback);
}

fn coin_thing() {
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

fn one_more_thing() {
    let x = 32;
    println!("x = {:?}\n", plus_one(Some(x)));
}

fn garden_thing() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!\n", plant);
}

fn vector_thingy() {
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

fn mapped_hashes_hash_mapping() {
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