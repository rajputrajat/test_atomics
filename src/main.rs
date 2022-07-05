use lazy_static::lazy_static;
use std::{
    sync::{Arc, Mutex},
    thread,
};

lazy_static! {
    static ref A: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

fn main() {
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());
    thread::spawn(|| simple_ops());

    println!("value of A: {}", *A.lock().unwrap());
}

fn simple_ops() {
    for _ in 0..1_000_000_000 {
        if *A.lock().unwrap() >= 0 {
            *A.lock().unwrap() -= 1
        }
        if *A.lock().unwrap() <= 0 {
            *A.lock().unwrap() += 1;
        }
    }
}
