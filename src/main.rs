use crate::fs::File;
use colored::Colorize;
use std::{
    env, fs,
    io::{self, BufRead},
};
use walkdir::WalkDir;

struct Config {
    search_term: Option<String>,
    file_paths: Vec<String>,
    args: Vec<String>,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        let mut new_config = Config {
            search_term: None,
            file_paths: Vec::new(),
            args: Vec::new(),
        };

        new_config.search_term = Some(args[1].clone());
        //let mut args_vec: Vec<String> = Vec::new();
        //let mut file_paths_vec: Vec<String> = Vec::new();

        for arg in &args[2..] {
            if arg.starts_with("-") {
                new_config.args.push(arg.clone());
            } else {
                new_config.file_paths.push(arg.clone());
            }
        }
        new_config
    }
}

struct Content {
    line_number: Option<u32>,
    line_string: String,
    file_path: Option<String>,
}

impl Content {
    pub fn new() -> Content {
        let new_content = Content {
            line_number: None,
            line_string: "".to_string(),
            file_path: None,
        };

        new_content
    }
}

fn main() {
    //get input args
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    //process input args into a config struct
    let config: Config = Config::new(&args);

    println!(
        "{:?}\n{:?}\n{:?}",
        config.search_term, config.file_paths, config.args
    );

    //check that a file path was provided
    let file_path: &String = match config.file_paths.first() {
        None => panic!("No filepath provided!"),
        Some(path) => path,
    };

    let file = open_file(file_path);

    //read lines to a Bufreader
    let file_line = io::BufReader::new(file).lines();
}

fn open_file(file_path: &String) -> File {
    let contents = File::open(file_path);

    let contents = match contents {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {error:?}"),
    };

    contents
}
