fn is_prime(n :i32) -> bool{
    let prime : bool = true;
    for i in 2..=(n/2) {
        if n % i == 0{
            return false; 
        }
    }
    true
}

fn main(){
    let args : Vec<String> = std::env::args().collect();
    let num:i32 = args[1].trim().parse().unwrap();

    println!("is it a prime number : {}", is_prime(num));
}