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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 10,
        Coin::Quarter => 25
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