pub fn use_match(){
    let age = 18; 
    let food: String = String::from("Pizza");  
    // match = switch statement 
    match age {
        18 => println!("Man, you're legal now!"), 
        _ => println!("Oops, too young")
    }

    match food {
        Pizza => println!("I love pizza nuggets!"), 
        Burger => println!("I love burger nuggets!"), 
        _ => println!("No food is available.")
    }
}