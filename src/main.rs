mod string_file;

use std::env::args;
use std::fmt::{format, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use atty::Stream;
use colored::Colorize;
use string_file::StringFile;
use walkdir::WalkDir;
use crate::string_file::LineNr;


fn get_command() -> (String, String, bool) {
    if atty::is(Stream::Stdin) {
        return get_from_args()
    }
    get_from_stint()
}

fn get_from_stint() -> (String, String, bool) { 
    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    if input.is_empty() {
        println!("No input provided via pipe");
        std::process::exit(1);
    }
    let query = args().skip(1).next().unwrap_or_default();
    (input.join("\n"), query.to_string(), true)
}
 fn get_from_args() -> (String, String, bool) {
     let args: Vec<String> = args().collect();
     if args.len() < 3 {
         panic!("Usage: {} <path> <query>", args[0]);
     }
     let path = &args[1];
     let query = &args[2];
     (path.to_string(), query.to_string(), false)
 }

fn main() {
    let (input, query, is_stin) = get_command();
    let result = match is_stin {
        true => find_query_in_stin(&input, &query),
        false => get_all_files(&input, &query) 
    };    
    print_files(&result);
}

fn find_query_in_stin(input: &String, query: &String) -> Vec<StringFile> {
    let mut result: Vec<StringFile> = Vec::new();
    let mut string_file: Vec<LineNr> = Vec::new();
    let mut index = 0;
    for line in input.split("\n") {
        collect_lines(&mut string_file, &line.to_string(), query, index);
        index += 1;
    }
    result.push(StringFile {
        path: "pipe".to_string(),
        contents: string_file,
    });
    result
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
        collect_lines(&mut string_file, &line.unwrap_or_else(|_| String::new()), query, index);
        index += 1;
    }
    if string_file.is_empty() {
        return;
    }
    string_file_list.push(StringFile {
        path: file_path.clone(),
        contents: string_file,
    })
}

fn collect_lines(string_file: &mut Vec<LineNr>, line: &String, query: &str, index: usize) {
    if let Some(start) = line.to_lowercase().find(&query.to_lowercase()) {
        let end = start + query.len();
        let highlighted_line = format!(
            "{}{}{}",
            &line[..start],
            &line[start..end].red().bold(),
            &line[end..]
        );
        string_file.push(LineNr {
            line: highlighted_line,
            number: index,
        });
    }
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