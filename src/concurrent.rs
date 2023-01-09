use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn multi_th_demo() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn move_vec_ref_demo() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || println!("Here's a vector: {:?}", v));

    handle.join().unwrap();
}

pub fn channel_demo() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got {}", received);
}
