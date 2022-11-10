use std::env; 

pub fn use_agrs(){
    let args: Vec<String> = env::args().collect(); 

    for argument in args.iter(){
        println!("{}", argument); 
    }

    // vector arguments
    println!("{:?}", args);
}