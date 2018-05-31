use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[cfg(test)]
mod test;

// data returned by the search function will live as long as the data passed into the search function in the contents argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result: Vec<&'a str> = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         // do something with line
    //         result.push(line);
    //     }
    // }
    // result
    contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|l| l.to_lowercase().contains(&query)).collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // // cloning the data also makes our code very
        // // straightforward because we don’t have to manage the
        // // lifetimes of the references; in this circumstance,
        // // giving up a little performance to gain simplicity
        // // is a worthwhile trade-off.
        // let query = args[1].clone();
        // let filename = args[2].clone();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}