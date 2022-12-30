use std::collections::HashMap;

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

pub fn mode_tester() {
    let v: Vec<i32> = [3,4,4,7,7,8,8,8,9,0,1,3].to_vec();
    println!("The mode of v is: {}.", get_mode(v));
}