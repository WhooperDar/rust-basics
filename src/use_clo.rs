// closures
pub fn use_closure() {
    println!("====== Closures ======");
    let n3: i32 = 10;

    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    let sub_nums = |n1: i32, n2: i32| n1 - n2;
    let divide_nums = |n1: i32, n2: i32| n2 / n1;

    println!("C sum is {}", add_nums(3, 2));
    println!("C sum is {}", sub_nums(3, 2));
    println!("C sum is {}", divide_nums(3, 2));
}