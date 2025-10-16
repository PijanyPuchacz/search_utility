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
    line_number: Option<usize>,
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

    //dbg!(&args);

    //check for help "-h" or "--help" input
    if args[1] == "-h" || args[1] == "--help" {
        print!(
            "Usage: grep [OPTIONS] <pattern> <files...>

        Options:
        -i                Case-insensitive search
        -n                Print line numbers
        -v                Invert match (exclude lines that match the pattern)
        -r                Recursive directory search
        -f                Print filenames
        -c                Enable colored output
        -h, --help        Show help information\n"
        );
        return;
    }

    //process input args into a config struct
    let config: Config = Config::new(&args);

    /*
    println!(
        "{:?}\n{:?}\n{:?}",
        config.search_term, config.file_paths, config.args
    );
    */

    //check that search term was provided
    let search_term = match config.search_term {
        None => panic!("No search term provided!"),
        Some(search_term) => search_term,
    };

    //convert form String -> str, needed for later pattern matching
    let search_term = search_term.as_str();

    //check that a file path exists
    match config.file_paths.first() {
        None => panic!("No filepath provided!"),
        Some(path) => path,
    };

    let mut content_vec: Vec<Content> = Vec::new();

    //loop through each path provided
    for file_path in config.file_paths {
        let file = open_file(&file_path);

        //read lines to a BufReader
        let file_lines = io::BufReader::new(file).lines();

        //search file for content
        for (line_number, line_string) in file_lines.map_while(Result::ok).enumerate() {
            if line_string.contains(search_term) {
                let new_content = Content {
                    line_number: Some(line_number),
                    line_string: line_string,
                    file_path: Some(file_path.clone()),
                };

                content_vec.push(new_content);
            }
        }
    }

    //print found lines
    for content in content_vec {
        println!(
            "{} | {}: {}",
            content.file_path.unwrap(),
            content.line_number.unwrap(),
            content.line_string
        );
    }
}

fn open_file(file_path: &String) -> File {
    let contents = File::open(file_path);

    let contents = match contents {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {error:?}"),
    };

    contents
}
