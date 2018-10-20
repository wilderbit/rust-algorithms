use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!{"from spawn {}", i};
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("from main {}", i);
        thread::sleep(Duration::from_millis(100))
    }

    handle.join().unwrap();
}
