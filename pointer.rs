
enum pointer{
    cons(i32,Box<pointer>),
    Nil,
}

use crate::pointer::{cons as c, Nil};

fn main(){
    let hello = c(1,Box::new(c(2,Box::new(c(3,Box::new(Nil))))));

    println!("hello - {:?}", hello);
}