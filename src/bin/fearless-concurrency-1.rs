// Write a Rust program that spawns two threads, each of which prints out a message to the console.
use std::thread;

fn main(){

    let h1 = thread::spawn(move || {
        println!("Hello from thread 1");
    });

    let h2 = thread::spawn(move || {
        println!("Hello from thread 2");
    });

    h1.join().unwrap();
    h2.join().unwrap();
}