use std::env; // "env" module in "std" library is used to work with command line arguments.
use std::fs; // "fs" module in "std" library is used to work with files.
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if let handles error from result type
    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let contents = fs::read_to_string(config.filename)?; 

    println!("With text:\n  {}", contents);

    Ok(())
}

struct Config{
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone(); // let filename = &args[2] also works
    
        Ok(Config { query, filename})
    }
}
