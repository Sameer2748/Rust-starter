// use std::{thread, time::Duration};

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hello from spawned  thread {}", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     // we can use the handle to wait for the spawned thread to finish
//      handle.join().unwrap();
//     for i in 1..5 {
//         println!("hello from main thread {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     // we let the spawned thread to finnish  and using this unwrap becuase it rentrun a result type 
//     // handle.join().unwrap();
// }


use std::thread;


fn main(){
    let v = vec![1,2,3,4,5];

    // this code will give an error becuase is owned by main function and the closure dont knwo till when v will live 
    // let handle = thread::spawn(||{
    //     println!("hello from spawned thread {:?}", v);
    // });

    // to fix this we can use move keyword which will take ownership of v and move it to the closure
    let handle = thread::spawn(move ||{
        println!("hello from spawned thread {:?}", v);
    });

    // cannot use c here because v is moved to the closure and is no longer available in main
    // println!("{:?}", v);


    handle.join().unwrap();
}