// fn main(){
//     let str1 = String::from("small");
//     let ans;
//     {
//         let str2 = String::from("largest");
//         ans = largest(&str1, &str2);
//     }

//     println!("{}", ans);
// }

// fn largest<'a>(str1: &'a str, str2: &'a str)-> &'a str{
//     if str1.len() > str2.len() {
//         return str1;
//     }
//     return str2;
// }


/// struct with lifetimes
struct User<'a> {
    name: &'a str
}
// struct User {
//     name: &str
// }

fn main() {
    let user;
    {
        let name = String::from("Sameer");
         user = User {name: &name};
    }


}