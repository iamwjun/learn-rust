use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, sx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("val is {}", val);
        tx.send(val).unwrap();
    });

    let received = sx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, sx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in sx {
        println!("Got: {}", received);
    }
}
