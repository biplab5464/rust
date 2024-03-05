fn main(){
    let var = vec![1,2,3];
    let mut iter = var.iter();
    let hemlo = 6;

    for v in  0..hemlo{
        println!("Got: {:?} \n var : {:?}", iter.next(),var);
    }

    let multiply = var.iter().map(|x| x * 3).collect::<Vec<i32>>();
    println!("vec : {:?}", multiply);

    let filter: Vec<_> = multiply.into_iter().filter(|x| *x < 6 ).collect();
    println!("filter :{:?}",filter);
}