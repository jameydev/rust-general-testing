struct Excerpt<'a> {
    part: &'a str
}

pub fn some_text() {
    let novel = String::from("The Wheel weaves as the Wheel wills...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let excerpt = Excerpt {
        part: first_sentence
    };

    println!("\n{}", excerpt.part);
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}