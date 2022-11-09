pub fn formatting() {

    println!("====== Formatting ====== ");
    // Formating
    println!("Hello I'm Kim Darius, future blockchain dev!");

    // Basic Formating
    println!(
        "{} is a web dev by morning & {} by night",
        "Kim Darius", "blockchain Dev"
    );

    // Positional Arguments
    println!(
        "{0} loves coding, {0} writes javascript and now {1} for his {2}.",
        "Darius", "Rust", "online job"
    );

    // name arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "baseball"
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    //prints tuple
    println!("{:?}", (12, true, "Darius"));

    // Basic math formatting
    println!("10 + 10 = {}", 10 + 10);
}
