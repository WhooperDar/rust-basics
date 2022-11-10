pub fn use_trait() { 
    let bird = Bird  {name, attack: 5}; 
    bird.print_name(); 

}

// struct 
struct Bird { 
    name: String, 
    attack: u64
}

impl Bird { 
    fn print_name(&self){ 
        println!("{}", self.name); 
    }
}