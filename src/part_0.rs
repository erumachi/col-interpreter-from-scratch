use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;

// Assignment 1:
// Write a function that takes in a string. Return:
//     The same string,
//     All lowercase,
//     Stripped of whitespaces.
fn cleanup_string(input: &str) -> String {
    let mut output = input.to_lowercase();
    output.retain(|ch| !ch.is_whitespace());
    output
}

// Assignment 2
// Write a function that takes in a string. Return:
//     The same string,
//     All lowercase,
//     Stripped of whitespaces,
//     Without any comments.
fn cleanup_and_strip_comments(input: &str) -> String {
    match cleanup_string(input)
        .split("//")
        .nth(0) {
            Some(x) => x,
            None => ""
        }.to_string()
}

// Assignment 3
// Return:
//     The same string,
//     All lowercase,
//     Stripped of whitespaces,
//     Without any comments.
// If the final result contains any characters that aren’t part of our language (a-z), throw an error.
fn parse_line(input: &str) -> Option<String> {
    let clean_input = cleanup_and_strip_comments(input);
    if clean_input.chars().all(|ch| ch.is_ascii_lowercase()) {
        return Some(clean_input);
    }
    None
}

// Assignment 4
// Write a function that takes in a file name.
// Return a single string, with each line in the file concatenated. For each line in the file, apply the function that you got from Assignment 3.
fn parse_file(filename: &str) -> Option<String> {
    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);

    let mut program: String = "".to_string();
    for line in reader.lines() {
        let parsed_line = parse_line(&(line.ok()?))?;
        program.push_str(&parsed_line);
    }
    Some(program)
}

// Final Assignment
// Write a function that takes in two strings as arguments. If the first item is equal to “run”, call the function that you wrote in Assignment 3 using the second argument.
// Else if the first item is equal to “file”, call the function that you wrote in Assignment 4 using the second argument.
// Else, throw an error.
fn run_or_file(command: &str, cmd_arg: &str) -> Option<String> {
    match command {
        "run" => {
            parse_line(cmd_arg)
        },
        "file" => {
            parse_file(cmd_arg)
        },
        _ => {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print!("Usages:\n\t{} run <expression>\n\t{} file <filename>\n", args[0], args[0]);
        return;
    }
    match run_or_file(&args[1], &args[2]) {
        Some(x) => println!("{}", x),
        None => println!("Error: operation failed")
    };
}
