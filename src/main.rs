
mod formatting; 
mod usevar; 
mod use_types;
mod use_str; 
mod use_tuples;
mod use_arr;
mod use_vec;
mod use_cont;
mod use_arrow;
mod use_clo; 
mod use_pointers;
mod use_structs;
mod use_trait; 
mod use_enums; 
mod use_hashmap;
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
    use_clo::use_closure();
    use_pointers::use_pointer();
    use_structs::use_structs();
    use_trait::use_trait();
    use_enums::use_enum();
    use_hashmap::use_hashmap();
}

pub fn test_run(){ 
    println!("Hello from the run function");

}
