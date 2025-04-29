mod string_file;
use std::fs::File;
use colored::Colorize;
use walkdir::WalkDir;
use crate::string_file::StringFile;
use crate::string_file::LineNr;
use std::io::{BufRead, BufReader};

pub fn java_api(input: &String, query: &String) -> Vec<StringFile> {
    get_all_files(input, query)
}

pub fn find_query_in_stin(input: &String, query: &String) -> Vec<StringFile> {
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

pub fn print_files(files: &[StringFile]) {
    for file in files {
        println!("{}", format!("File: {}", file.path).white().bold());
        for line in &file.contents {
            println!("\t {}", format!("{}", line).green());
        }
    }
}

pub fn get_all_lines_with_query(file_path: &String, query: &str, string_file_list: &mut Vec<StringFile>) {
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

pub fn collect_lines(string_file: &mut Vec<LineNr>, line: &String, query: &str, index: usize) {
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

pub fn get_all_files(path: &str, query: &String) -> Vec<StringFile> {
    let mut file_with_query = Vec::new();
    for line in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if line.file_type().is_file() {
            let file_path = line.path().to_str().unwrap_or_default();
            get_all_lines_with_query(&file_path.to_string(), query, &mut file_with_query);
        }
    }
    file_with_query
}
