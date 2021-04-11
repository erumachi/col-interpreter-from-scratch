use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;
use std::result::Result;

fn cleanup_string(input: &str) -> String {
    let mut output = input.to_lowercase();
    output.retain(|ch| !ch.is_whitespace());
    output
}

fn cleanup_and_strip_comments(input: &str) -> String {
    match cleanup_string(input)
        .split("//")
        .nth(0) {
            Some(x) => x,
            None => ""
        }.to_string()
}

fn parse_line(input: &str) -> Option<String> {
    let clean_input = cleanup_and_strip_comments(input);
    if clean_input.chars().all(|ch| ch.is_ascii_lowercase()) {
        return Some(clean_input);
    }
    None
}

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

fn interpret_program(memory: &mut Option<i64>, program: &str) -> Result<(), String> {
    for ch in program.chars() {
        match ch {
            'a' => {
                // Add/Overwrite the value in memory by 0
                *memory = Some(0);
            },
            'd' => {
                // Decrement the value in memory by 1.
                match memory {
                    Some(x) if *x > 1 => *x -= 1,
                    Some(_x) => return Err(String::from("cannot decrement: value should stay between 0 and 1000")),
                    None => return Err(String::from("cannot decrement an uninitialized value")),
                }
            },
            'i' => {
                // Increment the value in memory by 1.
                match memory {
                    Some(x) if *x < 999 => *x += 1,
                    Some(_x) => return Err(String::from("cannot increment: value should stay between 0 and 1000")),
                    None => return Err(String::from("cannot increment an uninitialized value")),
                }
            },
            'x' => {
                // Print the value in memory as an integer.
                match memory {
                    Some(x) => print!("{}", x),
                    None => return Err(String::from("cannot print an uninitialized value")),
                }
            },
            'y' => {
                // Remove the value in memory.
                match memory {
                    Some(_x) => *memory = None,
                    None => return Err(String::from("memory already uninitialized, cannot remove its value")),
                }
            },
            'z' => {
                // Exit the program.
                return Ok(());
            },
            _ => {
                // For now, simply ignore unknown characters
            },
        };
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print!("Usages:\n\t{} run <expression>\n\t{} file <filename>\n", args[0], args[0]);
        return;
    }
    let mut memory: Option<i64> = None;
    let program = match run_or_file(&args[1], &args[2]) {
        Some(x) => x,
        None => {
            println!("Error: operation failed");
            return;
        }
    };

    match interpret_program(&mut memory, &program) {
        Ok(_) => {},
        Err(s) => {
            println!("Execution error: {}", s);
        }
    };
}
