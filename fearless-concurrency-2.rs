/*
Race condition detection: 
Write a Rust program that creates two threads and increments a shared counter in each thread. 
Use a Mutex to synchronize access to the counter and print the final value of the counter after the threads have completed.
Modify the program to intentionally create a race condition by not using the Mutex, 
and observe the final value of the counter in this case.
Concepts: Mutex, synchronization, data races.
*/

use std::thread;
use std::sync::{Mutex, Arc};
fn main() {

    let mut handles = vec![];
    let m = Arc::new(Mutex::new(0));
    for _ in 0..2 {
        let m_cloned = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = m_cloned.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }    
    for handle in handles{
        handle.join().unwrap();
    }
    println!("{:?}", m);

}