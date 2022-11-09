pub fn use_data_types() {
    println!("====== Data Types ====== ");
    // primitive types of rust
    // Integers: i8, i16, i32, i128, u8, u16, u32, u128
    // Floats: f32, f64
    // Characters: char (single characters)
    // Boolean: bool
    // Tuples
    // Arrays [fixed size]
    // Vectors (dynamic size)

    // default type i32
    let x = 1;
    let y = 2;

    // default type = f64
    let _floatme = 26.5;

    println!("Default type of _floatme is f64 = {}", _floatme);

    println!("1 + 2 = {}", x + y);

    // explicit typing
    let value: i32 = 232323;
    let tempt: f64 = 32.5;
    let indexchar: char = 'Z';
    let indexchar2: char = 'B';

    println!("{}", value);
    println!("{}", tempt);
    println!("{}", indexchar);
    println!("{}", indexchar2);

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater: bool = 10 > 5;

    println!("{} {}", is_active, is_greater);

    // character
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{}, {}", a1, face);
}
