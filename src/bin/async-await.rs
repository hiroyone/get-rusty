// use futures::executor::block_on;

use std::time::Duration;

use tokio::time::sleep;

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}

async fn my_function(i: i32) {
    println!("[{i}] I'm an async function!");
    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");

    let s2 = read_from_database().await;
    println!("[{i}] Second result: {s2}");
}

#[tokio::main]
async fn main(){
    let mut handles = vec![];
    
    for i in 0..100 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    println!("Hello Rusty!");

    for handle in handles {
        handle.await.unwrap();
    }
}

// #[tokio::main]
// async fn main(){
//     // block_on(my_function());
//     let f = my_function(0);
//     println!("Hello Rusty!");
//     f.await;
// }