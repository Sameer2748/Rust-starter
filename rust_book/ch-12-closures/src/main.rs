use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32)-> u32{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(3));
    intensity
}

fn main(){
    let simulated_intensity =10;
    let simulated_round_number =7;

    generate_workout(simulated_intensity, simulated_round_number );
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher <T> 
where T: Fn(u32)-> u32,{
    fn new(calculation: T)-> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32 )-> u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);  
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number:u32){
    // let exercisse = simulated_expensive_calculation(intensity);
    // now we can use closure instead of this excersie call for expensive funtions 

    let mut exercise =  Cacher::new(|x| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        x
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", exercise.value(intensity));
        println!("Next, do {} situps!",  exercise.value(intensity));
    } else {
        if random_number == 3 {
            println!("Today, a break from exercise!");
        } else {
            println!("Today, run for {} minutes!",  exercise.value(intensity));
        }
    }
}