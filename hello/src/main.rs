use std::net::TcpListener;

fn main(){
    //this will bind the url and port to this program and store it in listner
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    // this will iterate thought 
    for stream in listner.incoming(){
        let stream = stream.unwrap();

        println!("connection instablished!");
    }
}