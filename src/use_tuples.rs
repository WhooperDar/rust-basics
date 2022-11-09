pub fn use_tuples(){ 
    println!("====== Tuples ====== "); 

    let notes: (&str, &str, i8) = ("header", "body", 10);
    
    println!("The {} and {}, lastly the size is {}", notes.0, notes.1, notes.2);

    let person: (&str, &str, i8) = ("bread", "Mass", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    // Another tuple
    let mut programming: (&str, &str, &str) = ("Rust", "Javascript", "TypeScript");

    // Another print tuple
    println!(
        "Another tuple programming langs 2022: {} {} {}",
        programming.0, programming.1, programming.2
    );

    // Mutated
    programming.2 = "C#";

    println!(
        "Another tuple programming langs 2022: {} {} {}",
        programming.0, programming.1, programming.2
    );
}