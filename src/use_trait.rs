
struct Bird { 
    name: String, 
    attack: u64
}

struct Workout { 
    workout: String,
    reps: i32, 
    sets: i32
}

impl Workout { 
    fn createWorkout(&self){
        println!("Workout: {}, reps: {}, sets: {}", self.workout, self.reps, self.sets);
    }
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
    fn is_carnivore(&self) -> bool {
        true
    }
}

// trait -> inteface 
trait Animal { 
    fn can_fly(&self) -> bool; 
    fn is_carnivore(&self) -> bool; 
    fn is_animal(&self) -> bool { 
        true
    }
}

pub fn use_trait() { 
    let name = String::from("Bird"); 

    let bird: Bird = Bird {name , attack: 5}; 
    bird.print_name(); 

    println!("{}", bird.can_fly()); 
    println!("{}", bird.is_carnivore()); 


    // new Workouts 
    let workout: Workout = Workout { workout: String::from("Bench Press"), reps: 15, sets: 3 }; 
    workout.createWorkout();
}
