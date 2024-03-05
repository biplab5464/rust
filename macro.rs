//rustc +nightly -Zunpretty=expanded macro.rs

macro_rules! s {
    ($e:expr) => {
        String::from($e)
    };
}

macro_rules! times_five {
    ($e:expr) => {
        5 * $e
    };
}

macro_rules! multiply_add {
    ($a : expr, $b : expr, $c : expr) => {
        $a * ($b + $c)
    };
}

macro_rules! vec_strs{
(
        //start a reptiton;
        $(
            $element:expr
        )
        //separeted by commas
        ,
        //zero or more time
        *
) =>{
    //enclose the expansion in a blcok so that we can use multiple statement
        {
            let mut v = Vec::new();
            // start a repetition;
            $(
                //Each reptition will contain the following statement with
                // $element replaced with the corresponding expression.
                v.push(format!("{}",$element));
            )*
            v
        }
    };

}

macro_rules! repeat_two{
    ($($i : ident)* , $($i2:ident)*) =>{
        $(let $i: (); let $i2 : ();)*
    }
}

fn main() {
    let world = s!("world");

    let _20 = times_five!(4);

    let mul_add = multiply_add!(1, 2, 3);

     let s = vec_strs![1, "a", true, 3.14159f32];
     
     repeat_two!( a b c d e f, u v w x y z );

    println!("Hello, {}!", world);
}
