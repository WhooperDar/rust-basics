pub fn use_enum() { 
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5); 
    let c: MyEnum = MyEnum::C{x:30, y: 50}; 
    let d: MyEnum = MyEnum::D(6); 
    
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);

    // Extracting the values
    if let MyEnum::B(val) = b { 
        println!("B = {}", val);
    }

    if let MyEnum::C{x,y} = c { 
        println!("X = {}, Y = {}", x,y );
    }


}
// enum 
#[derive(Debug)]
enum MyEnum {
    A, 
    B(i32), 
    C { x: i32, y: i32}, 
    D(i64),
}