mod string_file;

use std::env::args;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::Colorize;
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
    // let (path, query) = get_command();
    let (path, query) = mock_get_command();
    let files_with_query = get_all_files(&path, &query);
    print_files(&files_with_query);
}

fn print_files(files: &[StringFile]) {
    for file in files {
        println!("{}", format!("File: {}", file.path).white().bold());
        for line in &file.contents {
            println!("\t {}", format!("{}", line).green());
        }
    }
}

fn get_all_lines_with_query(file_path: &String, query: &str, string_file_list: &mut Vec<StringFile>) {
    let mut string_file: Vec<LineNr> = Vec::new();
    let file = File::open(&file_path).expect("File not found");
    let reader = BufReader::new(file);
    let mut index = 0;
    for line in reader.lines() {
        collect_lines(&mut string_file, &line.unwrap_or_else(|err| String::new()), query, index);
        index += 1;
    }
    string_file_list.push(StringFile {
        path: file_path.clone(),
        contents: string_file,
    })
}

fn collect_lines(string_file: &mut Vec<LineNr>, line: &String, query: &str, index: usize) {
    if !line.to_lowercase().contains(query.to_lowercase().as_str()) {
        return;
    }
    string_file.push(LineNr {
        line: line.clone(),
        number: index,
    });
}

fn get_all_files(path: &str, query: &String) -> Vec<StringFile> {
    let mut file_with_query = Vec::new();
    for line in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if line.file_type().is_file() {
            let file_path = line.path().to_str().unwrap_or_default();
            get_all_lines_with_query(&file_path.to_string(), query, &mut file_with_query);
        }
    }
    file_with_query
}