mod stack;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;
use std::result::Result;
use crate::stack::Stack;

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

fn interpret_program(memory: &mut Stack<i64>, program: &str) -> Result<(), String> {
    for ch in program.chars() {
        match ch {
            'a' => {
                // Pushes 0 to the top of the stack
                memory.push(0)?;
            },
            'b' => {
                // Pops the top item from the stack.
                memory.pop().ok_or("b error: stack is empty")?;
            },
            'd' => {
                // Decrements the top item of the stack by 1.
                match memory.last_mut().ok_or("d error: stack is empty")? {
                    x if *x > 0 => *x -= 1,
                    _ => return Err(String::from("cannot decrement: value should stay between 0 and 1000")),
                }
            },
            'i' => {
                // Increments the top item of the stack by 1.
                match memory.last_mut().ok_or("i error: stack is empty")? {
                    x if *x < 1000 => *x += 1,
                    _ => return Err(String::from("cannot increment: value should stay between 0 and 1000")),
                }
            },
            'q' => {
                // Duplicates the top item on the stack.
                let elem = (*(memory.last().ok_or("q error: stack is empty")?)).clone();
                memory.push(elem)?;
            },
            'v' => {
                // Increments the top item on the stack by 5.
                match memory.last_mut().ok_or("v error: stack is empty")? {
                    x if *x < 995 => *x += 5,
                    _ => return Err(String::from("cannot increment: value should stay between 0 and 1000")),
                }
            },
            'w' => {
                // Decrements the top item of the stack by 5.
                match memory.last_mut().ok_or("w error: stack is empty")? {
                    x if *x > 5 => *x -= 5,
                    _ => return Err(String::from("cannot decrement: value should stay between 0 and 1000")),
                }
            },
            'x' => {
                // Prints the top item on the stack as an integer.
                let elem = memory.last_mut().ok_or("x error: stack is empty")?;
                print!("{}", elem);
            },
            'y' => {
                // Deletes the entire stack.
                memory.clear();
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
    let max_stack_size = 100;
    let mut memory: Stack<i64> = Stack::new(max_stack_size);
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
