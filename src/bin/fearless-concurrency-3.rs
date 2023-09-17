/*
Message passing: 
Implement a message passing system using Rust channels or the crossbeam library. 
Use multiple threads to send and receive messages between them, 
and implement some form of synchronization or coordination between the threads.
Concepts: Message passing, channels, crossbeam, thread communication.
 */

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let num_threads = 4;
    let mut handles = vec![];

    // Spawn threads to send messages
    for i in 0..num_threads {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let msg = format!("Thread {} sent message {}", i, j);
                tx_clone.send(msg).unwrap();
            }
        });
        handles.push(handle);
    }

    // Spawn a thread to receive messages
    let handle = thread::spawn(move || {
        let mut received_msgs = 0;
        while received_msgs < num_threads * 5 {
            let msg = rx.recv().unwrap();
            println!("Received message: {}", msg);
            received_msgs += 1;
        }
    });
    handles.push(handle);

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
