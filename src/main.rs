mod string_file;

use std::env::args;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use string_file::StringFile;
use walkdir::WalkDir;
use crate::string_file::LineNr;

fn get_command() -> (String, String) {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("Usage: {} <path> <query>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    let query = &args[2];
    if path.is_empty() || query.is_empty() {
        println!("Path and query cannot be empty");
        std::process::exit(1);
    }
    (path.to_string(), query.to_string())
}

fn mock_get_command() -> (String, String) {
    let path = "/home/daniele/Documenti/APPDEV/txt_finder/test";
    let query = "Lorem Ipsum";
    (path.to_string(), query.to_string())
}

fn main() {
    //let (path, query) = get_command();
    let (path, query) = mock_get_command();
    let all_file = get_all_files(&path);
    let files_with_query = get_all_files_with_query(&all_file, &query);
    print_files(&files_with_query);
}

fn print_files(files: &[StringFile]) {
    for file in files {
        println!("File: {}", file.path);
        for line in &file.contents {
            println!("\t {}", line);
        }
    }
}

fn get_all_files_with_query(file_path_list: &[String], query: &str) -> Vec<StringFile> {
    let mut string_file_list: Vec<StringFile> = Vec::new();
    for file_path in file_path_list {
        let mut string_file : Vec<LineNr> = Vec::new();
        let file = File::open(&file_path).expect("File not found");
        let reader = BufReader::new(file);
        let mut index = 0;
        for line in reader.lines() {
            let line = line.expect("Error reading line");
            if !line.contains(query) {
                continue;
            }
            index += 1;
            string_file.push(LineNr {
                line: line.clone(),
                number: index,
            });
        }
        string_file_list.push(StringFile {
            path : file_path.clone(),
            contents: string_file,
        })
    }
    string_file_list
}

fn get_all_files(path: &str) -> Vec<String> {
    WalkDir::new(&path)
        .into_iter()
        .map(|entry| {
            entry.unwrap_or_else(|err| {
                println!("Error: {}", err);
                std::process::exit(1);
            })
        })
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            entry
                .path()
                .to_str()
                .expect("Filename not UTF-8")
                .to_owned()
        })
        .collect::<Vec<_>>()
}
