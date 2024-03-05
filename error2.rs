use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main(){
    read_username_from_file();
}