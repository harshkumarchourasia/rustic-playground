/*
Write a Rust program that spawns two threads and has them both read and write to a shared hashmap. 
Make sure the hashmap is synchronized properly so that both threads can access it without conflicts.
 */

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    for i in 0..2 {
        let map = shared_map.clone();
        let handle = thread::spawn(move || {
            let mut map = map.lock().unwrap();
            map.insert(i, i*2);
            println!("Thread {} inserted key {} value {}", i, i, i*2);
            let value = map.get(&i);

            match value {
                Some(v) => println!("Thread {} read ({}, {})", i, i, v),
                None => println!("Thread {} read ({}, <not found>)", i, i),
            }

        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }

}