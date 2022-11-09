use std::mem;

pub fn use_arrays() {
    // Arrays = Fixed Sized
    // Type  size = values
    println!("====== Arrays ====== ");

    // static arrays
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val
    println!("Single val: {}", numbers[0]);

    numbers[0] = 5;

    println!("Single val: {}", numbers[0]);

    // Array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array oocupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    // Reference from other file
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);

    let slice: &[i32] = &numbers[0..1];
    println!("Slice: {:?}", slice);
}
