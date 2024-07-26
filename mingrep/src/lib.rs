use std::fs;
use std::error::Error;
use std::env;

/// the Error below needs to be pulled into the scope
pub fn run(config: Config) -> Result <(), Box<dyn Error>> {
    // let filedata = fs::read_to_string(config.filename)
        // .expect("The file required is not present in directory");
    // println!("Query : {}", config.query);
    // println!("Query : {}", config.filename);
    let filedata = fs::read_to_string(config.filename)?;

    // println!("{}", filedata);
    // for line in search(&config.query, &filedata) {
        // println!("{}", line);
    // }
    let results = if config.case_sensitive {
        search(&config.query, &filedata)
    } else {
        search_case_insensitive(&config.query, &filedata)
    };
    if results.len() > 0 {
        for line in results{
            println!("{}", line);
        }
    } else {
        println!("There is no matching output");
    }
   Ok(())
}
// ' is coming from shift + "
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    // vec![]
    for line in content.lines(){
        if line.contains(query){
            result.push(line.trim());
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    // vec![]
    let query = query.to_lowercase();
    for line in content.lines(){
        if line.to_ascii_lowercase().contains(&query){
            result.push(line.trim());
        }
    }
    result
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn parse_args(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("args are not enough");
            return Err("args are not enough");
        }
        // cover the returned Config to Results
        Ok(Config { 
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err()
        })
        // env::var() will check if the env var is set, or return error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let content = "\
    Rust:
    safe, fast, productive
    Pick Three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, content));
    }
    #[test]
    fn two_result(){
        let query = "trus";
        let content = "\
    Rust:
    safe, fast, productive
    Pick Three.
    Trust all Rust Libraries";
        assert_eq!(vec!["Trust all Rust Libraries"],
                search_case_insensitive(query, content));
    }
}