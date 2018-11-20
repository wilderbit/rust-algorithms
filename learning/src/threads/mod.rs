use std::{sync::mpsc, thread, time::Duration};

fn simple_thread() {
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println! {"from handle2 {}", i};
            thread::sleep(Duration::from_millis(100));
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..10 {
            println! {"from handle1 {}", i};
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("from main {}", i);
        thread::sleep(Duration::from_millis(100))
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn move_closure(list: &'static [i32]) {
    thread::spawn(move || {
        println!("{:?}", list);
    });

    thread::spawn(move || {
        println!("{:?}", list);
    });

    thread::spawn(move || {
        println!("{:?}", list);
    });
}

fn message_passing_data() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    thread::spawn(|| {
        for msg in rx {
            println!("{}", msg);
        }
    });
}

fn message_passing_multithreaded() {
    let (tx, rx) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();
    let handler1 = thread::spawn(move || {
        let h1 = thread::spawn(|| {
            for msg in rx1 {
                println!("{}", msg);
            }
        });

        let h2 = thread::spawn(move || {
            let val = String::from("Hi");
            loop {
                thread::sleep_ms(100);
                tx.send(val.clone()).unwrap();
            }
        });
        h1.join().unwrap();
        h2.join().unwrap();
    });

    let handler2 = thread::spawn(|| {
        let h1 = thread::spawn(|| {
            for msg in rx {
                println!("{}", msg);
            }
        });

        let h2 = thread::spawn(move || {
            let val = String::from("Hello");
            loop {
                thread::sleep_ms(100);
                tx1.send(val.clone()).unwrap();
            }
        });
        h1.join().unwrap();
        h2.join().unwrap();
    });

    handler1.join().unwrap();
    handler2.join().unwrap();
}

fn multiple_producer_single_consumer() {
    #[derive(Debug)]
    struct Abc {
        id: String,
    };
    let (tx, tr) = mpsc::channel();
    let new_tx = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let v = vec![
            Abc {
                id: "1".to_string(),
            },
            Abc {
                id: "2".to_string(),
            },
            Abc {
                id: "3".to_string(),
            },
        ];
        for number in v {
            tx.send(number).unwrap();
        }
    });

    thread::spawn(move || {
        let v = vec![
            Abc {
                id: "1".to_string(),
            },
            Abc {
                id: "2".to_string(),
            },
            Abc {
                id: "3".to_string(),
            },
        ];
        for number in v {
            new_tx.send(number).unwrap();
        }
    });

    for number in tr {
        let number: Abc = number;
        println!("{:#?}", number.id);
    }
}

fn main() {
    //    simple_thread();
    //    static ABC: &'static [i32] = &[1, 2, 3, 4, 5];
    //    move_closure(&ABC);
    multiple_producer_single_consumer();
    thread::sleep_ms(10000);
}
