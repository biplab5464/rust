use minigrep::Config;

fn main() {
    let main_config = Config::build(std::env::args()).unwrap_or_else(|err|{
        eprintln!("Prblem parsing arugumerts : {err}");
        std::process::exit(1);
    });

    //println!("Query - {} \nFile Path - {}",main_config.query,main_config.file_path);
    if let Err(e) = minigrep::run(main_config){
        println!("application error: {e}");
        std::process::exit(2)
    };
    
}


