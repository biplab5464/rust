use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let greeting_file_result = File::open("helllo1.txt");
    //println!("========> {:?}",greeting_file_result);

    let _greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(err) => match err.kind(){
            ErrorKind::NotFound => File::create("hello1.txt").expect("problem creating file"),
            _ => panic!(" problem openling file")
        }
    };
}