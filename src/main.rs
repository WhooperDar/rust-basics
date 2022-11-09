
mod formatting; 
mod usevar; 
mod use_types;
mod use_str; 
mod use_tuples;
mod use_arr;

fn main() {
    println!("Hello, world!");

    test_run(); 
    formatting::formatting();
    usevar::usevariables(); 
    use_types::use_data_types();
    use_str::use_strings();
    use_tuples::use_tuples();  
    use_arr::use_arrays();
}

pub fn test_run(){ 
    println!("Hello from the run function");
}
