// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(4);
//     vec.push(5);
//     // vec.push("3");

//     let len = vec.len();
//     let mut evenvec = Vec::new();

//     for (index, number) in vec.iter().enumerate() {
//         if number % 2 == 0 {
//             evenvec.push(number);
//         }
//     }

//     println!("{}", len);

//     println!("{:?}", vec);
//     println!("{:?}", evenvec);
// }


// hashMaps
// fn main(){

//     let mut users : HashMap<String, i32> = HashMap::new();

//     users.insert(String::from("manish"), 20); 
//     users.insert(String::from("sameer"), 20); 

//     let  user= users.get("manish");

//     match user {
//         Some(age) => println!("{}", age),
//         None => println!("no value")
//     }

//     // update the key value if present and return the prev value of that key 
//     let prev = users.insert(String::from("manish"), 30);
//     let prev2 = users.insert(String::from("manish"), 40);

//     println!("{:?} {:?}", prev, prev2);

// }

// assignment which return a hashmap form the tuple passed to group 
// // this convert the vector of tuples of string and integer into a hashmp of string and integer 
// fn group_val_key(vec : Vec<(String, i32)>) -> HashMap<String, i32>{

//     let mut hm = HashMap::new();
//     for (key, value) in vec {
//         hm.insert(key, value);
//     }
//     return hm;
// }

// fn main(){
//     let input_vec = vec![(String::from("manish"), 23), (String::from("sameer"), 21)];

//     let hm = group_val_key(input_vec);

//     println!("{:?}", hm);
// }


// iteratoors
// fn main(){
//     let mut v1 = vec![1,2,3,4,5];

//     // this is borrowing the values 
//     // for val in v1.iter() {
//     //     println!("{}", val);
//     // }

//     // // for loop
//     // for val in v1 {
//     //     println!("{}", val);
//     // }

//     // let v1_mut = v1.iter_mut();
//     // //iter mut wanna get the muttable value 
//     // for val in  v1_mut {
//     //     *val = *val + 1
//     // }

//     // we have this iterator this next returns a option if none or some value 

//     let mut mut_vec = v1.iter_mut();

//     while let Some(val) = mut_vec.next() {
//         println!("{}", val);
//     }
//     println!("{:?}", v1);
// }

// // assignment fildert odd values and double each one and return a new vector
// fn main(){
//     let vec = vec![1,2,3,4,5];

//     let new_vec = odd_double_vector(vec);

//     println!("{:?}", new_vec);
// }


// fn odd_double_vector(vec : Vec<i32>)-> Vec<i32>{
//     let mut newvec = vec![];

//     // filter odd values 
//     let filteredvalues = vec.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);

//     for i in filteredvalues {
//         println!("{}", i);
//         newvec.push(i);
//     }
//     return newvec;
// }

// traits
trait Summary{
    fn summarize(&self) -> String;
    fn welcome(&self) -> String;
}
struct User {
    name:String,
    age: u32
}
impl Summary for User{
    fn summarize(&self) -> String {
     return format!("user is {}, age is {}", self.name, self.age);
    }
    fn welcome(&self) -> String {
        return format!("Welcome! {}", self.name);
    }
}
fn main(){
    let user = User{
        name: String::from("Sameer rao"),
        age: 21
    };

    user.summarize();
    println!("{}", user.summarize());
    println!("{}", user.welcome());
}



