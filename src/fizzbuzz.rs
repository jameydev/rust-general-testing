pub fn fizz_buzz(n: usize) {
    let mut count = 1;
    while count <= n {
        if count % 3 == 0 && !(count % 5 == 0) {
            println!("Fizz");
        }
        else if count % 5 == 0 && !(count % 3 == 0) {
            println!("Buzz");
        }
        else if count % 3 == 0 && count % 5 == 0 {
            println!("FizzBuzz");
        }
        else {
            println!("{count}");
        }
        count += 1;
    }
}