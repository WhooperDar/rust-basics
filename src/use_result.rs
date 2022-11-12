
#[derive(Debug)]
enum MyError{
    Error 
}
// divivde helper function 
fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError>{
    if dividend % divisor != 0 { 
        Err(MyError::Error) // return error if fails
    } 
    else {
        Ok(dividend / divisor) // return value if success
    }
}

pub fn use_result(){
    let divide1: Result<i32, MyError> = divide(15, 3);
    let divide2: Result<i32, MyError> = divide(666, 23232);

    match divide1 {
        Ok(v) => println!("Value from result = {}", v),
        Err(v) => println!("{:?}", v)
    }

    match divide2 { 
        Ok(v) => println!("Divide 2 {}", v), 
        Err(MyError) => println!("Erorr 2 {:?}", MyError)
    }
}