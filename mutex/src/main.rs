use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let m = a.clone();
        let handle = thread::spawn(move || {
            let mut x = m.lock().unwrap();
            *x += 1;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("m = {}", a.lock().unwrap());
}
