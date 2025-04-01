use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug )]

struct User {
    username:String,
    password:String
}
// macro for these

fn main(){
 let user  = User{
    username : String::from("Sameer rao"),
    password : String::from("12343412")
 };

 let serilize_string = serde_json::to_string(&user);
 // we get result of this serialize 
 match serilize_string {
    Ok(str)=> println!("serial {}", str),
    Err(_)=> print!("Error while converting to String")
 }

 let user2 = String::from("{\"username\": \"Sameer\", \"password\": \"1234\"}");

 let desserialize : Result<User, serde_json::Error> = serde_json::from_str(&user2);
 match desserialize {
    Ok(str)=> println!(" deserial {:?}", str),
    Err(_)=> println!("error vkas,")
 }
}



