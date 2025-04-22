use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

struct StringFile {
    path: String,
    contents: Vec<String>,
}

fn main() {
    let path = args().nth(1).expect("No query given").trim().to_owned();
    let query = args().nth(2).expect("No query given").trim().to_owned();
    let all_file = get_all_files(&path);
    let files_with_query = get_all_files_with_query(&all_file, &query);
    print_files(&files_with_query);
}

fn print_files(files: &[StringFile]) {
    for file in files {
        println!("File: {}", file.path);
        for line in &file.contents {
            println!("\t{}", line);
        }
    }
}

fn get_all_files_with_query(file_path_list: &[String], query: &str) -> Vec<StringFile> {
    let mut string_file_list: Vec<StringFile> = Vec::new();
    for file_path in file_path_list {
        let file = File::open(&file_path).expect("File not found");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Error reading line");
            if !line.contains(query) {
                continue;
            }
            string_file_list.push(StringFile {
                path: file_path.to_owned(),
                contents: vec![line],
            })
        }
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
