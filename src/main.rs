// find the sum from 1 to 10^8 using threads

use::std::{
    sync::mpsc,
    thread::{self, spawn}
};

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000..(i + 1) * 10000000 {
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx); // close the channel
    let mut final_sum = 0;
    for val in rx {
        println!("Received: {}", val);
        final_sum = final_sum + val;
    }
    println!("Final sum: {}", final_sum);
    
}