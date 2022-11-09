
mod formatting; 
mod usevar; 
mod use_types;
mod use_str; 
mod use_tuples;
mod use_arr;
mod use_vec;
mod use_cont;
mod use_arrow;

fn main() {
    println!("Hello, world!");

    test_run(); 
    formatting::formatting();
    usevar::usevariables(); 
    use_types::use_data_types();
    use_str::use_strings();
    use_tuples::use_tuples();  
    use_arr::use_arrays();
    use_vec::use_vectors();
    use_cont::use_conditionals();
    use_arrow::use_arrow_fn();
}

pub fn test_run(){ 
    println!("Hello from the run function");
}
