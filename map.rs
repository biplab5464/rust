/**
 * DESC : map x value in range a - b to a different range m - n of value
 * @param m : i32 - min range of the value m - n 
 * @param n : i32 - max range of the value m - n
 * @param m : i32 - min range of the value a - b 
 * @param n : i32 - max range of the value a - b
 * @param x : i32 - x range in the value  a - b 
 * @return -> i32  - return the value y in range m - n
 */
fn map(m: i32, n : i32 , a : i32, b : i32,  x : i32  ) -> i32{
    if m >  n {
        panic!("n can't be less then m");
        //return -1 
    }
    if a >  b {
        panic!( "b can't be less then a");
        //return -1 
    }
    if x < a || x > b {
        panic!("x can't be less then a and can't be greater then b")
    }

    return ( ( x - a ) * ( n - m )) / ( b - a ) + m ;
}

fn main(){
    
    let args : Vec<String> = std::env::args().collect();
    let num_str = &args[1];
    let num : i32 = num_str.trim().parse().unwrap();

    let value = map(1,25,7,98,num);
    println!("mapped value : {} ", value );
}