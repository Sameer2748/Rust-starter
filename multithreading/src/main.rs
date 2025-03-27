// use std::thread;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..5000000 {
//             println!("i is from spwaned {}", i);
//         }
//     });

//     //  run after only spawned thread is stopped 
//     handle.join().unwrap();

//     for i in 1..50000000 {
//         println!("i is main core {}", i);
//     };
// }


// fn main(){
//     let v = vec![1,2,3];
//     let handle = thread::spawn(move || {
//        println!("{:?}", v);            
//     }); 
//     handle.join().unwrap();
// }


use std::thread;
use std::sync::mpsc;

fn main(){
    let (tx, rx) = mpsc::channel();

    // spawn a thread 
    thread::spawn(move || {
        let msg = String::from("hi");
        // we use unwrap so first the thread will  finish work then next code with work 
        tx.send(msg).unwrap();
    });

    let recieved = rx.recv().unwrap();
    println!("got msg {}", recieved);
}


