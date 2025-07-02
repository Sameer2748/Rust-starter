use std::env;
use std::fs;
// we use thjos process module ffor exting the code without any panic or noise in logs 
use std::process; 
use std::error::Error;

fn main() {
    let args: Vec<String> =  env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("problme parsing arguments, {}", err);
        process::exit(1);
    });
    println!("query {}", config.query);
    println!("for file {}", config.filename);

    if let Err(e) = run(config){
        println!("Application Eror{} ", e);
        process::exit(1);
    };
}

fn run(config: Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("{}", contents);
    Ok(( ))
}

struct Config{
    query: String,
    filename:String
}

impl Config{ 
    fn new(args: &[String])-> Result<Config, &str>{
        if args.len() < 3{
            return Err("args not defined");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config {query, filename}) 
    }
}
