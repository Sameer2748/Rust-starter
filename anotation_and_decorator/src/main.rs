// // macros
// macro_rules! say_hello {
// ()=>{
//     println!("hello by Sameer");
// }
// }

// fn main() {
//     say_hello!();
//     println!("Hello, world!");
// }

// use std::fmt::{Debug};

// #[derive(Debug)]

// struct User {
//     username : String
// }

// fn main(){
//     let u = User{
//         username: String::from("SaMEER RAO")
//     };
//     println!("{:?}", u);
//     panic!("heyyy hooo")
// }

//copynand clone macros

// fn main(){
//     let user =  String::from("vdksv");

//     // for strings we transfer thenownnership and for numbers we copy values
//     //  things that are on stack gey clone but on heap they are not cloned
//     //we can use copy for stack hut for heap we use clone

//     // 1 this clone is expensive sop avoid this  for strings
//     let user2 = user.clone();

//     // 2 way to do this is by passing the value but returning and getting it from the function
//     let user3  = print_it(user);

//     println!("{}", user3);

// }
// fn print_it(a: String) -> String{
//     println!("{}", a);
//     return a;
// }

// coy and clone

// #[derive(Debug)]
#[derive(Debug, Copy, Clone)]

struct User {
    is_male: bool,
    age: i32,
}

fn main() {
    let u1 = User {
        is_male: true,
        age: 22,
    };
    let u2 = u1;

    println!("{:?}, {:?}", u1, u2);
}
