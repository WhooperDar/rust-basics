pub fn usevariables() {

    println!("====== Variables ====== ");
    // immutable value
    let name = "Darius";

    // mutable value
    let mut age = 21;
    println!("My name is {0}, and i am age {1}", name, age);
    age = 39;
    println!("My name is {0}, and i am age {1}", name, age);

    let mut lastname = "darius";
    println!("{0}", lastname);

    lastname = "Kim Darius";
    println!("{0}", lastname);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    const ID_2: i32 = 002;
    println!("ID 2: {}", ID_2);

    // multiple assign variables at once
    let (my_name, my_age) = ("Darius", 21);

    println!("{} is {}", my_name, my_age);
}
