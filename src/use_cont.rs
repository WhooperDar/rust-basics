pub fn use_conditionals() {
    let ages = 51;

    // if else
    if ages >= 21 {
        println!("Bartender: What would you like to drink?",);
    } else {
        println!("Bartender: Sorry, you have to leave.");
    }

    greetings("Hi", "Darius");
    buy_things("Cars", 55.6, true);

    // check odd or even basic 
    let ran_number: [i32; 4] = [57, 23, 12, 10]; 

    for x in ran_number.iter(){ 
        if x % 2 == 0 { 
            println!("EVEN!"); 
        }
        else { 
            println!("ODD!"); 
        }
    }
}

// subprocess -> helper function
pub fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

pub fn buy_things(things: &str, value: f64, isWritten: bool) {
    if isWritten {
        println!("I need to buy this: {} {}", things, value);
    } else {
        println!("I need to buy this: {} {}", things, value);
    }
}