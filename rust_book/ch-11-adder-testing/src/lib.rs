#[derive(Debug)]

struct Rectangle {
    width: u32,
    height:u32
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width  && self.height > other.height
    }
}

pub fn add_two(num: i32)-> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold_test(){

        let large = Rectangle {
            width: 30,
            height:30
        };
        let small = Rectangle{
            width:20,
            height:20
        };
        assert!(large.can_hold(&small)); 

    }

    #[test]
    fn check_add(){
        println!("{:?}", add_two(2));
    }

}
