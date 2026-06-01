use std::{env, fs, process};
use std::error::Error;
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    case_insensitive: bool,
}

impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let case_insensitive = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query: &args[1], file_path: &args[2], case_insensitive: case_insensitive})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    return Ok(());
}
