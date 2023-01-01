use std::collections::HashMap;

fn nth_value(v: Vec<i32>, n: i32) -> i32 {
    let mut result = 0;
    let mut numbers = &v;
    let mut under_pivot = Vec::new();
    let mut over_pivot = Vec::new();
    let mut equal_pivot = Vec::new();

    // choose pivot
    let index = n / 2;
    let pivot = numbers.get(index as usize);

    // push values to appropriate sub vectors
    for i in numbers {
        if pivot < Some(&0) {
            under_pivot.push(*i);
        }
        else if pivot > Some(&0) {
            over_pivot.push(*i);
        }
        else {
            equal_pivot.push(*i);
        }
    }

    // get the result recursively
    if n < under_pivot.len() as i32 {
        result = nth_value(under_pivot, n);
    }
    else if n < (under_pivot.len() + equal_pivot.len()) as i32 {
        result = *pivot.unwrap();
    }
    else {
        result = nth_value(over_pivot, n - under_pivot.len() as i32);
    }

    return result;
}

fn get_median(v: Vec<i32>) -> i32 {
    let mut numbers = v;
    let n = numbers.len();

    let mut mid = 0;
    if numbers.len() % 2 == 0 {
        mid = nth_value(numbers, (n - 1) as i32);
    }
    else {
        mid = nth_value(numbers, n as i32);
    }

    return mid;
}

fn get_mode(v: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    for (key, value) in map {
        if value > mode {
            mode = *key;
        }
    }

    return mode;
}

pub fn median_tester() {
    let v = [81, 32, 5, 73, 43, 98, 3].to_vec();

    println!("The values in v: ");
    for i in &v {
        println!("{i}");
    }

    println!("\nThe median of v is: {}.", get_median(v));
}

pub fn mode_tester() {
    let v: Vec<i32> = [3,4,4,7,7,8,8,8,9,0,1,3].to_vec();
    println!("Values in v: ");
    for i in &v {
        println!("{i}");
    }
    println!("\nThe mode of v is: {}.", get_mode(v));
}