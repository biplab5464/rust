fn main(){
    let args : Vec<String> = std::env::args().collect();
    let num : u64 = args[1].trim().parse().unwrap();
    let mut prime : u64 = 2;
    let mut new_prime = num;
    let mut prime_factor : Vec<u64> = vec![];

    while new_prime > 1{
        //println!("new prime {}",new_prime );
        if  new_prime % prime == 0 {
            //println!("if prime {}",prime);
            prime_factor.push(prime);
            new_prime /= prime;
         }else if prime >= 3{
            //println!("else if prime {} + 2",prime);
            prime += 2 ;
        }else {
            //println!("else prime {} + 1",prime);
            prime += 1;
        }
        //println!("=================================")
    }
    println!("{:?}" ,prime_factor);
}