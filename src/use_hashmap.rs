use std::collections::HashMap; 

pub fn use_hashmap() { 
    let mut map = HashMap::new(); 
    
    map.insert(0, "Hi"); 
    map.insert(1, "Hi2"); 

    println!("{:?}", map);

    match map.get(&0){
        Some(str) => println!("{}", str), 
        None => println!("Doesn't exist in map."),
    }

    match map.get(&1){
        Some(str) => println!("{}", str), 
        None => println!("Doesn't exist in map."), 
    }

    map.remove(&0); 
    println!("{:?}", map);

}