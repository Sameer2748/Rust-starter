use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug )]

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

 let mut v: Vec<u8> = Vec::new();

 // converted in bytes 
 let ans = user.serialize(&mut v);

 println!("{:?}", v);

 // converted into origional json data 
 let u = User::try_from_slice(&v).unwrap();
 println!("{}", user.username);

}



