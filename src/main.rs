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
            loop {
                if A.compare_exchange(0, 1, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            }
        }
        {
            loop {
                if A.compare_exchange(1, 0, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            }
        }
        {
            loop {
                if A.compare_exchange(0, -1, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            }
        }
        {
            loop {
                if A.compare_exchange(-1, 0, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            }
        }
    }
}
