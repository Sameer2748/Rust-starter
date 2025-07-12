// mpsc stand for "multiple producer, single consumer"
use std::sync::mpsc;
use std::{thread, time::Duration};


// This is a simple example of using channels in Rust to send messages between threads
fn main(){
    let (tx, rx)= mpsc::channel();
    let (tx2, rx2)= mpsc::channel();

    let handle = thread::spawn(move || {
        let v = vec![String::from("I love nancy"),String::from("I love sunita"),String::from("I love memta"),String::from("I love sushma")];
        for i in v {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }

    });

    for val in rx {
        println!("Received message: {}", val);
    };


    let handle2 = thread::spawn(move || {
        let v = vec![String::from("I love ssakshi"),String::from("I love nanika"),String::from("I love priyanka"),String::from("I love kalpana")];
        for i in v {
            tx2.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }

    });

    for val in rx2 {
        println!("Received message: {}", val);
    };

  
    handle.join().unwrap();
    handle2.join().unwrap();
}