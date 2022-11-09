
// usual structs (like c)

// decleration
struct Color { 
    red: u8, 
    green: u8, 
    blue: u8
}

struct Person { 
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person { 
        Person { 
            first_name: first.to_string(), 
            last_name: last.to_string()
        }
    }
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

    // use case of person 
    let mut person = Person::new("Lebron", "James"); 

    println!("First name: {}, Second name: {}", person.first_name, person.last_name);



}   