// fn main() {
//     let v1 = vec![1, 2, 3];

//     let viter = v1.iter();

//     for val in viter{
//         println!("{}", val);
//     }
// }


// #[test]
// fn iterator_sum1(){
//     let v1 = vec![1,2,3];
//     let v1_iter = v1.iter();

//     let v1_sum :i32 = v1_iter.sum();

//     assert_eq!(v1_sum, 6);

// }
// #[test]
// fn iterator_sum(){
//     let v1 = vec![1,2,3];
//     let v1_iter : Vec<_> = v1.iter().map(|x| x + 1).collect();

//     assert_eq!(v1_iter, vec![2, 3, 4]);

// }



// #[derive(PartialEq, Debug)]
// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn check_shoe(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|shoe| shoe.size == size).collect()
// }

// fn main(){
//     let shoe = vec![
//         Shoe { size: 10, style: String::from("sneaker") },
//         Shoe { size: 13, style: String::from("sandal") },
//         Shoe { size: 10, style: String::from("boot") },
//     ];

//     let size_vec = check_shoe(shoe, 10);

//     assert_eq!(size_vec , vec![
//         Shoe { size: 10, style: String::from("sneaker") },
//         Shoe { size: 10, style: String::from("boot") },
//     ]);
// }


struct Counter {
    count : u32
}


impl Counter {
    fn new() -> Counter{
        Counter {count:0}
    }
}

impl Iterator for Counter {
    // this is associated type for the Iterator trait
    // it defines the type of items that the iterator will yield
    // in this case, it is u32
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test(){
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn main(){

}
