// // fn main() {
// //     // println!("{}", is_even(20));
// //     // println!("{}", fib(5));

// //     let str = String::from("sdvksksdv");
// //     let  len = str_length(str);
// //     println!("{}", len);
// // }

// // fn is_even(num: i32) -> bool{
// //     if num % 2 ==  0 {
// // return true;
// //     }else {
// //         return false;
// //     }
// // }

// // fn fib(num : u32) -> u32{

// //     let mut first  = 0;
// //     let mut second = 1;

// //     if num == 0 {
// //         return 0;
// //     }
// //     if num == 1 {
// //         return 1;
// //     }

// //     for _ in 0..num - 1 {
// //         let temp = second;
// //         second = first+second;
// //         first = temp;
// //     }

// //     return second;
// // }

// // fn str_length(str: String) -> usize {
// //     return str.chars().count();
// // }

// // adding functions to a struct 

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }
// }

// fn main() {
//     let rect = Rect {
//         width: 30,
//         height: 40,
//     };
//     println!("Area is {}", rect.area());
// }


// // enum 
// enum Shape {
//     Circle(f64),
//     Rectangle(f64),
//     Triangle(f64)
// }




// options for adding nulls, none chrono 
// fn main(){
//  let str = String::from("vssdfv");
//  let ans = find_A(str);

//  match ans{
//     Some(value) => println!("value is {}", value),
//     None => println!("value is None")
//  }

//  let greet = fs::read_to_string("hello.txt");

//  match greet {
//     Ok(content)=> {
//         println!("content is : {:?}", content);
//     }
//     Err(error)=> {
//         println!("No content is : {:?}", error);
//     }
//  }
// }

// fn find_A(str : String) -> Option<i32> {

// for (index,char) in str.chars().enumerate(){
//      if char =='a' {
//         return Some(index as i32);
//      }
// }
// return None;
// }

// use chrono::Utc;

// fn main() {
// let now = Utc::now();
// println!("current time is {}", now)
// }

fn main(){
    let mut a = String::from("hello");
    print_b(&mut a);
    println!("num is {}", a);
}
fn print_b(b: &mut String) {
    b.push_str(" sameer");
    println!("value of  b is {}", b);
}