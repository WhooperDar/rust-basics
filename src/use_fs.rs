use std::fs::File; 

use std::io::prelude::*; 

pub fn use_file_sys(){ 
   read_file(); 
   write_file();
}

// read file from text
fn read_file(){
     // opening a file 
     let mut file = File::open("index.txt").expect("Can't file open");

     let mut contents = String::new(); 

     // read text to string 
     file.read_to_string(&mut contents)
         .expect("Oops! can not read the file...."); 
 
     println!("File contents: {}", contents);
}

fn write_file(){ 
    let mut file = File::create("test.txt") 
        .expect("Error, can't create file");
    
    file.write_all(b"Writing this file to the test.txt file, hope it works 232")
        .expect("Can't write to the file, sorry!"); 
    
    println!("File created!");
}