// use chrono::prelude::*;
// use chrono::{Local, Utc};

// fn main() {
//     let utc = Utc::now();
//     let local = Local::now();
//     println!("Hello, world! {} , {}", utc, local);
// }

// env 
use dotenv::dotenv;
use std::env;

// fn main(){
//     dotenv().ok();

//     let var = env::var("Password");

//     match var {
//         Ok(str) => println!("{}", str),
//         Err(_e) => println!("Error while reading value")
//     }

// }

// for some generics value inputs we hzaave to give the traits bounds 
// fn main(){
//     let a = sum(5,3);

//     println!("{}", a);
// }

// fn sum<T: Ord>(a: T, b: T) -> T{
//     if a<b {
//         return a;
//     }

//     return b;
// }


// Generics over structs 

struct Rect<T>{
    width: T,
    height:T
}

// we are usign mul with copy trait because we are calling mul trait and passing the variavble it is shring them and it need some copy trait for varibale memory managemnent issue 
// we use copy so it dont take reference oif variables and  copy them to calculate
impl <T: std::ops::Mul<Output = T> + Copy> Rect<T>{
    fn area(&self)-> T{
        return self.width * self.height;
    }
}

fn main(){
    let  area = Rect{
        width:20,
        height:20
    };

    println!("{}", area.area());
}


