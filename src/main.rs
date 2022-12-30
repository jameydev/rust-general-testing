mod rectangle;
mod stuff;
mod user;

pub use rectangle::Rectangle;
pub use stuff::*;
pub use user::User;


fn main() {
    // let tup = (500, 6.4, 3);
    // let (x, y, z) = tup;
    // println!("\nThe values of tup are {x}, {y}, and {z}");

    // countdown(10);

    let s = String::from("mega-ultra chicken");

    let word = stuff::first_word(&s);

    println!("\nFirst word: {}\n", word);

    // let mut user1 = User {
    //     email: String::from("someperson@example.com"),
    //     username: String::from("someuser123"),
    //     active: true,
    //     sign_in_count: 1
    // };

    let user = User::build_user(String::from("jesus@god.damn"), String::from("jesuschristdude"));
    
    dbg!(&user);

    println!();

    // let w = 10;
    // let h = 2;
    // println!("The area is {} square pixels", area(w, h));

    // let rectangle: (u32, u32) = (10, 20);
    // println!("The area of the rectangle is {}px.", area(rectangle));

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

    // let home = IPAddress {
    //     kind: IpAddrKind::IPv4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IPAddress {
    //     kind: IpAddrKind::IPv6,
    //     address: String::from("::1")
    // };
    
    let home = IpAddress::IPv4(127, 0, 0, 1);
    let loopback = IpAddress::IPv6(String::from("::1"));

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