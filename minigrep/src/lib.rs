use std::fs;

pub struct Config{
    pub query : String,
    pub file_path : String,
    pub ignore_case : bool,
}

impl Config {
    pub fn build( mut args: impl Iterator<Item = String>) -> Result<Config , &'static str> {
    
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let mut ignore_case = false;
        // match  args.next(){
        //     Some(str) => {
        //         if str == "i" { ignore_case = true }
        //     },
        //     None => ignore_case = false
        // };
        
        if let Some(c) = args.next() {
            if c == "i" { ignore_case = true }
            else { ignore_case = false }
        };

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run( config: Config) -> Result<(),Box<dyn std::error::Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    //println!("with text : \" {}\"",contents);
    // for line in search(&config.query, &contents ){
    //     println!("{line}");
    // }

    let reuslts = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in reuslts {
        println!("{line}");
    }
    //println!("{:?}",reuslts);

    Ok(())
}

pub fn search<'a>(query : &str,contents : &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    
    contents
        .lines()
        .filter(|line | line.to_lowercase().contains(&query))
        .collect()

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query  = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three. 
Duck tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query,content));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(vec!["Rust:", "Trust me."],search_case_insensitive(query, contents));
    }

}