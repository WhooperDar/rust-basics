pub fn use_pointer(){ 
    // Primitive array 
    let arr1: [i32; 3] = [1, 2, 3]; 
    let arr2  = arr1; 

    println!("Array ref= {:?}", (arr1, arr2));

    // Non-primitives 
    let v1: Vec<i32> = vec![1, 3, 4]; 
    let v2 = &v1; 

    println!("Vector ref= {:?}", (&v1, v2));

}