use std::{fs, result};

pub struct Config{
    pub query : String,
    pub file_path : String,
    pub ignore_case : bool,
}

impl Config {
    pub fn build( args: &[String]) -> Result<Config , &'static str> {
        if args.len() != 3 {
            return Err("so We Need Two arguments \"File name\" and \"string\" ");
        }

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config { query: args[1].clone(), file_path: args[2].clone(), ignore_case  })
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

pub fn search<'a>(query : &str, content : &'a str) ->  Vec<&'a str>{
    let mut results = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>( query : &str, contents : &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line)
        }
    }
    results
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