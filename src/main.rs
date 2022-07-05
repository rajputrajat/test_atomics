use lazy_static::lazy_static;
use std::{
    sync::atomic::{AtomicIsize, Ordering},
    thread,
};

lazy_static! {
    static ref A: AtomicIsize = AtomicIsize::new(0);
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

    println!("value of A: {}", A.load(Ordering::Relaxed));
}

fn simple_ops() {
    for _ in 0..10_000_000_000_isize {
        {
            if A.load(Ordering::SeqCst) >= 0 {
                A.fetch_sub(1, Ordering::SeqCst);
            }
        }
        {
            if A.load(Ordering::SeqCst) < 0 {
                A.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}
