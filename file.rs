use std::fs::File;              //lib for File system management 
use std::io::prelude::*;        //reexport BufRead, Read, Seek, Write
//use std::path::Path;            //slice of a path

fn main(){

    // //1 Create a path to desire file
    // let path = Path::new("hello.txt").display(); //path to the file

    //2 opne the file in read mode, it return 'io::Results<File>
    let mut file_result = File::open("helllo.txt");  //taking the file to the scope it return a Result
    let mut file = match file_result{
        Ok(file) => file,
        Err(err) => panic!("{}",err)
    };

    //3 Read the content of the file into a String
    let mut string = String::new();  //Creating a file to store 
    let read_file = file.read_to_string(&mut string); //convert the file type to string

    match read_file{
        Ok(read_file) => println!("{}",read_file),
        Err(err) => panic!("{}",err)
    }
}