use std::thread;
use std::time;
use time::Duration;

fn main(){

    let child_thd = thread::spawn(||{
        for i in 0..10{
            println!("printing from spawn thread {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 0..5{
        println!("printing from main thread {i}");
        thread::sleep(Duration::from_millis(10));
    }

    match child_thd.join() {
        Ok(suc) => println!("{:?}",suc),
        Err(err) => println!("{:?}",err)
    }

}
