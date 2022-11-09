
// usual structs (like c)

// decleration
struct Color { 
    red: u8, 
    green: u8, 
    blue: u8
}

// Tuple struct
struct Color_tuple (u8, u8, u8); 

pub fn use_structs(){ 
    // implementation 
    let mut colorizer = Color {
        red: 255, 
        green: 123, 
        blue: 55
    }; 
    

    // using tuple struct 
    let mut colorizer_tuple: Color_tuple = Color_tuple(255, 33, 244);

    // accessing values
    println!("Color: {} {} {}", colorizer.red, colorizer.green, colorizer.blue);
    println!("Color: {} {} {}", colorizer_tuple.0, colorizer_tuple.1, colorizer_tuple.2 ); 
}   