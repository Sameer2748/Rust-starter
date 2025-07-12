// fn main() {
//     println!("Hello, world!");

//     // box smart pointer 
//     let x = Box::new(5);
//     println!("x = {}", x);


//     // dereferencing trait 

//     let x = 5;
//     // let y = &x;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     // now we cannot use the refernece integer to a integer
//     assert_eq!(5, y);
//     // so to fix thisk we use defrefernce opf integer 
//     assert_eq!(5, *y);


// }

use std::ops::Deref;

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// now letas implemntn the deref trait for our MyBox struct
impl <T> Deref for MyBox<T>{
    // we will use the associated type Target to specify the type that we want to dereference
    type Target = T;

    fn deref(&self) -> &T {
        // this is the first item in our tuple struct
        &self.0
    }
}

fn main(){

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // now we can dereference y to get the value of x
    assert_eq!(5, *y);
}


// drop trait 
// we can use drop(any value which we wanna drop befopre the end of scope)

// reference count 
// we can use Rc<T> to keep track of the number of references to a value
// this is useful when we want to share a value between multiple owners

// inetrior mutability




