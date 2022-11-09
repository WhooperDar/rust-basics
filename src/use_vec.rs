use std::mem;

pub fn use_vectors() {
    // Resizable Arrays

    println!("====== Vectors ====== ");

    // vector store 11, 22, 33
    let mut vector_store: Vec<i32> = vec![11, 22, 33, 44];

    println!("{:?} vector_store", vector_store);

    // Get single val
    println!("Single Value Array{}", vector_store[0]);

    // Get length array
    println!("Vector Length: {}", vector_store.len());

    // Vector are stack allocated
    println!("{}", mem::size_of_val(&vector_store));

    // Get slice
    let slice1: &[i32] = &vector_store[1..3];
    println!("Slice Vector: {:?}", slice1);

    // Loop through vector values (Read-only)
    for v in vector_store.iter() {
        println!("Number: {}", v);
    }

    // Loop & mutate values (Read and write)
    for x in vector_store.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", vector_store);
}