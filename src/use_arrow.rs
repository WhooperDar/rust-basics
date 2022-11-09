// use arrow func
pub fn use_arrow_fn() {
    println!("====== Arrow Functions ======"); 
    
    let get_sum = add(1, 2);
    let get_prod: i32 = multiply(1000,23); 
    let get_quo: i32 = divide(50, 222222);
    let get_sub: i32 = subtract(6000, 23455);

    println!("Sum = {}", get_sum);
    println!("Difference = {}", get_sub);
    println!("Product = {}", get_prod);
    println!("Quotient = {}", get_quo);
}

// adder
pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // return without semi colon
}

// difference 
pub fn subtract(n1: i32, n2: i32) -> i32 {
    n1 - n2 // return without semi colon
}

pub fn multiply(n1: i32, n2: i32) -> i32 { 
    n1 * n2  
}

pub fn divide(n1: i32, n2: i32) -> i32 { 
    n2 / n1
}