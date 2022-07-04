use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let a = Arc::new(Mutex::new(10));
    let b = Arc::new(Mutex::new(20));
    let c = Arc::new(Mutex::new(30));

    let aa = Arc::clone(&a);
    let bb = Arc::clone(&b);
    let cc = Arc::clone(&c);

    thread::spawn(move || {
        *aa.lock().unwrap() = 30;
        if *bb.lock().unwrap() == 30 {
            *cc.lock().unwrap() = 20;
        }
    });

    let aa = Arc::clone(&a);
    let bb = Arc::clone(&b);
    let cc = Arc::clone(&c);

    thread::spawn(move || {
        *bb.lock().unwrap() = 30;
        if *aa.lock().unwrap() == 10 {
            *cc.lock().unwrap() = 20;
        }
    });

    println!(
        "{}, {}, {}",
        *a.lock().unwrap(),
        *b.lock().unwrap(),
        *c.lock().unwrap()
    );
}
