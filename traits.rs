//********* Generic type in rust ************

fn print<T>( var : T){
    println!("You pass the value:  {}",  var );
}

fn main(){
    print(6);
    print('💕');
}