use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};

pub fn get_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error=> {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    // let mut username = String::new();
    // File::open("heck.txt")?.read_to_string(&mut username)?;
    // Ok(username)
    fs::read_to_string("hello.txt")
}
