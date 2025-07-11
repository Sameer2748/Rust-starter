use std::env;
// we use thjos process module ffor exting the code without any panic or noise in logs 
use std::process; 
use minigrep::Config; 

fn main() {
    let args: Vec<String> =  env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("problme parsing arguments, {}", err);
        process::exit(1);
    });
    println!("query {}", config.query);
    println!("for file {}", config.filename);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application Eror{} ", e);
        process::exit(1);
    };
}
