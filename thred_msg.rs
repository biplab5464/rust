use std::sync::mpsc;
use std::thread;


fn main(){
    let (tx,rx) = mpsc::channel();

    let tx2 = tx.clone();
    let tx3 = tx.clone();

    thread::spawn(move ||{
        tx.send(String::from("hello this is a message from thread")).unwrap_or_else(|err|{
            println!("{:?}",err);
        })
    });

    thread::spawn(move ||{
        tx2.send(String::from("hello this is a message from the second thread hey i am just thre seocond thread")).unwrap_or_else(|err|{
            println!("{:?}",err);
        })
    });
    thread::spawn(move ||{
        tx3.send(String::from("hello this is a message from the third thread you are sending messages")).unwrap_or_else(|err|{
            println!("{:?}",err);
        })
    });

    // println!("Got {}", rx.recv().unwrap());
    // println!("Got {}", rx.recv().unwrap());

    for rec in rx{
        println!("{}",rec);
    }


}
