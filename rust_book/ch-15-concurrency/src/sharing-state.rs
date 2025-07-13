use std::sync::{Arc, Mutex};
use std::thread;


fn main(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10000 {
        // we cannot use rc because it is not thread safe
        // so we use Arc which is an atomic reference counter which is thread safe
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    };

    for handle in handles {
        handle.join().unwrap();
    };

    println!("Result: {}", *counter.lock().unwrap());
}


