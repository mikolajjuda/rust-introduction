use std::{time::Duration, sync::{mpsc, Arc, Mutex}, thread};

fn main() {
    let m1 = Arc::new(Mutex::new(1));
    let m2 = m1.clone();
    let (tx, rx) = mpsc::channel();
    let t1 = thread::spawn(move || {
        for _ in 0..10 {
            {
                let mut num = m1.lock().unwrap();
                *num += 1;
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
    let t2 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(600));
            let num = m2.lock().unwrap();
            tx.send(*num).unwrap();
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
    t1.join().unwrap();
    t2.join().unwrap();
}
