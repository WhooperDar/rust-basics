
struct Bird { 
    name: String, 
    attack: u64
}



// struct 
impl Bird { 
    fn print_name(&self){ 
        println!("{}", self.name); 
    }
}

impl Animal for Bird  {
    fn can_fly(&self) -> bool {
        true
    }
}
// trait -> inteface 
trait Animal { 
    fn can_fly(&self) -> bool; 
    fn is_animal(&self) -> bool { 
        true
    }
}

pub fn use_trait() { 
    let name = String::from("Bird"); 

    let bird: Bird = Bird {name , attack: 5}; 
    bird.print_name(); 

    println!("{}", bird.can_fly()); 
}
