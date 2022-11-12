fn main(){

    let num_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(&num_list);
    println!("The largest number is {}", largest);

    let char_list = vec!["a", "b", "e", "g", "c"];
    let largest = get_largest(&char_list);
    println!("The largest number is {}", largest);
}

fn get_largest<T>(list: &Vec<T>) -> T 
    where T: PartialOrd + Copy {
    let mut largest = list[0];
    for num in list.iter() {
        if num > &largest {
            largest = *num;
        }
    }
    
    return largest
}
