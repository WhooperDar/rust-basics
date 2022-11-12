// This is an intermediate concept of rust programming language 
use std::fs::File; 
use std::io;
use std::io::{ErrorKind, Read}; 

pub fn use_error(){

    // unrecoverable errors with panic!
    // panic!("crash and burn!");

    // panic! backtrace
    // let v = vec![1, 2, 3]; 
    // v[88];

    // Recoverable Error Handling 
    let f = File::open("hello.txt"); 

    let f = match f {
        // match arm  1
        Ok(file) => file, 
        // match guard
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.text"){
                Ok(fc) => fc, 
                Err(e) => {
                    panic!("Tried to create file but there was a problem {:?}", e)
                },
            }
        },
        // match arm 2
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

}

// propagate error long method 
fn read_user_text_from_file() -> Result<String, io::Error> {
    let f = File::open("index.txt");

    let mut f = match f {
        Ok(file) => file, 
        Err(e) => return Err(e),
    };

    let mut s = String::new(); 

    match f.read_to_string(&mut s){
        Ok(_) => Ok(s), 
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: ?
fn read_user_text_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; 
    let mut s = String::new(); 
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Shorter handle error propagate in Rust
fn read_user_text_shorter() -> Result<String, io::Error> {
    let mut s = String::new(); 

    File::open("hello.txt")?.read_to_string(&mut s)?; 

    Ok(s)
}