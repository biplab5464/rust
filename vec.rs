fn main(){
    
    let mut vec = vec![10,20,30];

    vec.push(40);
    println!("vec.push(40) : {:?}",vec);

    match vec.pop(){
        Some(result) => println!("vec.pop() : {} \n{:?}",result, vec),
        None => println!("Vector is empty ")
    }
    
}