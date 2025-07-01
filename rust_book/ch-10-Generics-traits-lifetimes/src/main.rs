// struct Point<T,U> {
//     x:T,
//     y:U
// }

// fn main() {
//     // let p = Point {x:20,y:80};
//     // let p2 = Point {x:22.0,y:80.0};
//     // let p2 = Point {x:22.0,y:80};
// }



// fn main(){
//     enum Option<T>{
//         Some(T),
//         None
//     }

//     enum Result<T,E>{
//         Ok(T),
//         Err(E)
//     }
// }



// traits 
pub struct NewsArticle{
    pub author:String,
    pub headline:String,
    pub content:String,
}

impl Summary for NewsArticle{
    fn Summarize(&self) -> String{
        format!("{} , by {}", self.author, self.content)
    }
}
pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply: bool,
    pub retweet: bool
}
impl Summary for Tweet{
    fn Summarize(&self) -> String{
        format!("{} , by {}", self.username, self.content)
    }
}

pub trait Summary {
    fn Summarize(&self) -> String;
}

fn main(){
    let tweet = Tweet {
        username:String::from("helloworld@gmail.com"),
        content:String::from("helloworld"),
        reply: true,
        retweet: false
    };
    let article = NewsArticle {
        author:String::from("helloworld2@gmail.com"),
        headline:String::from("helloworld"),
        content:String::from("helloworld"),
    };

    println!("Tweet Summary, {}", tweet.Summarize());
    println!("Article Summary, {}", article.Summarize());

}