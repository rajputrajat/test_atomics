use lazy_static::lazy_static;
use std::{
    sync::{Arc, Mutex},
    thread,
};

lazy_static! {
    static ref A: Arc<Mutex<isize>> = Arc::new(Mutex::new(0));
}

fn main() {
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);
    thread::spawn(simple_ops);

    println!("value of A: {}", *A.lock().unwrap());
}

fn simple_ops() {
    for _ in 0..10_000_000_000_isize {
        {
            let mut v = A.lock().unwrap();
            if *v >= 0 {
                *v -= 1
            }
        }
        {
            let mut v = A.lock().unwrap();
            if *v < 0 {
                *v += 1
            }
        }
    }
}
