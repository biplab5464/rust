struct Point<T> {
    width : T,
    height : T,
}

impl<T: std::cmp::PartialOrd> Point<T> {

    fn can_contain(&self,other : Point<T>)-> bool{
        self.width < other.width && self.height > other.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let big = Point{
            width : 4,
            height : 7,
        };
        let small = Point{
            width : 3,
            height : 10,
        };
        assert!(big.can_contain(small));
    }
}
