use std::{thread, time::Duration};

fn main(){
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawn thread", i);
                thread::sleep(Duration::from_millis(10));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread", i);
            thread::sleep(Duration::from_millis(20));
        }

        handle.join().unwrap();

}