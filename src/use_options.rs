pub fn use_options(){ 
    let divide1: Option<i32> = divide(5, 1); 
    let divide2: Option<i32> = divide(4, 2); 

    println!("{:?} unwraps to {}", divide1, divide1.unwrap()); 
    println!("{:?} unwraps to {}", divide2, divide2.unwrap());
}

// returning an option, similar to js -> promises
fn divide(dividend: i32, divisor: i32) -> Option<i32> { 
    if dividend % divisor != 0 { 
        // when fail to solve
        None
    }
    else {
        // successful
        Some(dividend/ divisor)
    }
}