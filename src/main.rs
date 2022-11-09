
mod formatting; 
mod usevar; 
mod use_types;
mod use_str; 

fn main() {
    println!("Hello, world!");

    test_run(); 
    formatting::formatting();
    usevar::usevariables(); 
    use_types::use_data_types();
    use_str::use_strings(); 
}

pub fn test_run(){ 
    println!("Hello from the run function");
}
