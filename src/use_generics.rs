
// Generics in struct
#[derive(Debug)]
struct Point <T, U> {
    x: T, 
    y: U
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point <T,W> {
        Point {
            x: self.x, 
            y: other.y
        }
    }
}

struct Point1<T> {
    x: T, 
    y: T
}

// method decleration with Generic Type
impl<T> Point1<T> {
    fn x(&self) -> &T{ 
        &self.x
    }
}

// Generics in enums OPTION
enum Option<T>{
    Some(T), 
    None
}

// Generics in enums RESULT
enum Result<T,E> {
    Ok(T), 
    Err(E)
}

pub fn use_generics(){
    let point = Point {x: 5, y: 11.323};

    println!("{:?}", point);

    // point 1 
    let point1= Point1 {x: 5, y: 6}; 

    println!("{:?}", (point1.x(), point1.x()));

    // two different generic type
    let point_mixup1 = Point { x: 5, y: 10.4}; 
    let point_mixup2 = Point { x: "Hello", y: 'c'}; 

    // combining two generic types 
    let p3 = point_mixup1.mixup(point_mixup2); 

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}