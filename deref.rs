struct MyBox(i32);

impl MyBox {
    fn mew( x : i32 ) -> MyBox{
        MyBox(x)
    }
}

use std::ops::Deref;

impl Deref for MyBox{
    type Target = i32;
    fn deref(&self) -> &i32{
        &self.0
    }
}

fn main(){

    let y = 5;
    let x = MyBox::mew(y);

    assert_eq!(5 , y);
    assert_eq!(5 , *x )
    
}