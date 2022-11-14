
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
mod use_options; 
mod use_fs;
mod use_args;
mod use_match;  
mod use_result;
mod use_errorhandling;
mod use_generics;
mod program;


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
    use_options::use_options();
    use_fs::use_file_sys();
    use_args::use_agrs(); 
    use_match::use_match();
    use_result::use_result();
    use_errorhandling::use_error();
    use_generics::use_generics();
    program::weather();

}

pub fn test_run(){ 
    println!("Hello from the run function");

}
