use std::sync::Mutex;

fn main(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 4
    }

    println!("m = {:?}", m)
}

