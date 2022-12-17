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

fn handle_connection(mut stream :TcpStream){
    // let buf_reader = BufReader::new(&mut stream);

    // let http_request : Vec<_> = buf_reader.lines()
    //                                         .map(|result| result.unwrap())
    //                                         .take_while(|line| !line.is_empty())
    //                                         .collect();
     let status_line = "HTTP/1.1 200 OK";
     let contents = fs::read_to_string("hello.html").unwrap();
     let length = contents.len();
     let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length,contents);

     stream.write_all(response.as_bytes()).unwrap();
}