fn main(){
    
    let string : Vec<String> = std::env::args().collect();
    //println!("args : {:?}", string);
    
    if string.len() != 2 {
        panic!( "one one argument accepted");
    }

}
