use std::net::{TcpListener,TcpStream};
use std::io::{prelude::*,BufReader};
use std::fs;

fn main(){
    //this will bind the url and port to this program and store it in listner
    // the bind function works like new
    let listner = TcpListener::bind("127.0.0.1:7871").unwrap();

    // this will iterate thought request
    //icoming works like a iterator
    for (index,stream) in listner.incoming().enumerate(){  //
        let stream = stream.unwrap();
        handle_connection(stream);

        println!("connection instablished! {index}");
    }
}

/*
    *DESC - this function take a TcpStream, get the request line and then send back reply using tcp/ip

*/
fn handle_connection(mut stream :TcpStream){
     let buf_reader = BufReader::new(&mut stream);

     let request_line = buf_reader.lines().next().unwrap().unwrap();

     let (status_line, contents) = if request_line == "GET / HTTP/1.1"{
         ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap())

     }else{
         ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("404.html").unwrap())
     };
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line,contents.len(),contents );
     stream.write_all(response.as_bytes()).unwrap();

}