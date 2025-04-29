mod string_file;

use std::env::args;
use std::io::{self, BufRead};
use atty::Stream;
use txt_finder::print_files;
use txt_finder::get_all_files;
use txt_finder::find_query_in_stin;


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
