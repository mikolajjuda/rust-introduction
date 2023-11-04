use std::thread;
use std::time::Duration;

fn main(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("main thread: done");
    handle.join().unwrap();
    println!("main thread: joined");
}
