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

        //check for passed parameter "-r"
        if new_config.args.contains(&"-r".to_string()) {
            let mut temp_vec: Vec<String> = Vec::new();

            //use walkdir to find all filepaths needed
            //search for recursive paths in the provided paths
            for top_path in &new_config.file_paths {
                for entry in WalkDir::new(top_path) {
                    //push each path to temp vector
                    match entry {
                        Ok(dir) => temp_vec.push(dir.into_path().to_str().unwrap().to_string()),
                        Err(_) => panic!("Error retreiving recursive file paths."),
                    }
                }
            }

            //replace file_paths in config with all found recursive paths
            new_config.file_paths = temp_vec;
        }

        new_config
    }
}

struct Content {
    line_number: Option<usize>,
    line_vec: Vec<String>,
    file_path: Option<String>,
}

impl Content {
    pub fn new(
        line_number: Option<usize>,
        line_vec: Vec<String>,
        file_path: Option<String>,
    ) -> Content {
        let new_content = Content {
            line_number: line_number,
            line_vec: line_vec,
            file_path: file_path,
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
            "Usage: grep [OPTIONS] <pattern> <files...>\n
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

    //vector to store all found matches
    let mut content_vec: Vec<Content> = Vec::new();

    //loop through each path provided
    for file_path in config.file_paths {
        let file = open_file(&file_path);

        //read lines to a BufReader
        let file_lines = io::BufReader::new(file).lines();

        //search file for content
        for (line_number, mut line_string) in file_lines.map_while(Result::ok).enumerate() {
            //check for case insensitive "-i" or inverted "-v" parameter
            match config.args.contains(&"-i".to_string()) {
                true => match config.args.contains(&"-v".to_string()) {
                    true => {
                        //case insensitive && inverted
                        if !(line_string
                            .to_lowercase()
                            .contains(search_term.to_lowercase().as_str()))
                        {
                            let line_vec: Vec<String> = [line_string].to_vec();

                            let new_content: Content = Content::new(
                                Some(line_number + 1),
                                line_vec,
                                Some(file_path.clone()),
                            );
                            content_vec.push(new_content);
                        }
                    }

                    false => {
                        //case insensitive && standard
                        if line_string
                            .to_lowercase()
                            .contains(search_term.to_lowercase().as_str())
                        {
                            let mut line_vec: Vec<String> = Vec::new();

                            //get word location in line, used if "-c" is provided later
                            let word_location = line_string
                                .to_lowercase()
                                .find(search_term.to_lowercase().as_str())
                                .unwrap();
                            let first_split =
                                line_string.split_off(word_location + search_term.len());
                            let second_split = line_string.split_off(word_location);
                            line_vec.push(line_string); //first part of line
                            line_vec.push(second_split); //searched word here
                            line_vec.push(first_split); //second part of line

                            let new_content: Content = Content::new(
                                Some(line_number + 1),
                                line_vec,
                                Some(file_path.clone()),
                            );
                            content_vec.push(new_content);
                        }
                    }
                },
                false => match config.args.contains(&"-v".to_string()) {
                    true => {
                        //case sensitive && inverted
                        if !(line_string.contains(search_term)) {
                            let line_vec: Vec<String> = [line_string].to_vec();

                            let new_content: Content = Content::new(
                                Some(line_number + 1),
                                line_vec,
                                Some(file_path.clone()),
                            );
                            content_vec.push(new_content);
                        }
                    }
                    false => {
                        //case sensitive && standard
                        if line_string.contains(search_term) {
                            let mut line_vec: Vec<String> = Vec::new();

                            //get word location in line, used if "-c" is provided later
                            let word_location = line_string.find(search_term).unwrap();
                            let first_split =
                                line_string.split_off(word_location + search_term.len());
                            let second_split = line_string.split_off(word_location);
                            line_vec.push(line_string); //first part of line
                            line_vec.push(second_split); //searched word here
                            line_vec.push(first_split); //second part of line

                            let new_content: Content = Content::new(
                                Some(line_number + 1),
                                line_vec,
                                Some(file_path.clone()),
                            );
                            content_vec.push(new_content);
                        }
                    }
                },
            };
        }
    }

    //print found lines
    for content in content_vec {
        match config.args.contains(&"-f".to_string()) {
            true => match config.args.contains(&"-n".to_string()) {
                true => {
                    //filepath && line number
                    print!(
                        "{}| {}: ",
                        content.file_path.unwrap(),
                        content.line_number.unwrap(),
                    );
                    for (pos, line) in content.line_vec.iter().enumerate() {
                        if config.args.contains(&"-c".to_string()) && pos == 1 {
                            print!("{}", line.color("red"));
                        } else {
                            print!("{line}");
                        }
                    }
                    print!("\n");
                }
                false => {
                    //filepath
                    print!("{}: ", content.file_path.unwrap());

                    for (pos, line) in content.line_vec.iter().enumerate() {
                        if config.args.contains(&"-c".to_string()) && pos == 1 {
                            print!("{}", line.color("red"));
                        } else {
                            print!("{line}");
                        }
                    }
                    print!("\n");
                }
            },
            false => match config.args.contains(&"-n".to_string()) {
                true => {
                    //line number
                    print!("{}: ", content.line_number.unwrap());

                    for (pos, line) in content.line_vec.iter().enumerate() {
                        if config.args.contains(&"-c".to_string()) && pos == 1 {
                            print!("{}", line.color("red"));
                        } else {
                            print!("{line}");
                        }
                    }
                    print!("\n");
                }
                false => {
                    //no options

                    for (pos, line) in content.line_vec.iter().enumerate() {
                        if config.args.contains(&"-c".to_string()) && pos == 1 {
                            print!("{}", line.color("red"));
                        } else {
                            print!("{line}");
                        }
                    }
                    print!("\n");
                }
            },
        };
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
