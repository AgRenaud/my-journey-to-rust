use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MyWorker {
    pub value: u8,
}

impl MyWorker {
    pub fn new() -> MyWorker {
        MyWorker {
            value: 0,
        }
    }
}

fn main() {
    let w = Arc::new(Mutex::new(MyWorker::new()));
    let mut handles = vec![];

     for _ in 0..10 {
         let w = Arc::clone(&w);
         let handle = thread::spawn(move || {
             let mut worker = w.lock().unwrap();
 
             worker.value += 1;
         });
         handles.push(handle);
     }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("w = {:?}", w);
}
