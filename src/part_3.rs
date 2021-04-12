mod stack;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;
use std::result::Result;
use crate::stack::Stack;
use std::io;
use std::io::Read;

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
            'c' => {
                // Subtracts the 2nd item on the stack from the top item and pushes the result to the stack.
                let top = memory.pop().ok_or("c error: stack is empty")?;
                let second = memory.pop().ok_or("c error: stack is empty")?;
                let result = second - top;
                memory.push(second)?;
                memory.push(top)?;
                memory.push(result)?;
            },
            'd' => {
                // Decrements the top item of the stack by 1.
                match memory.last_mut().ok_or("d error: stack is empty")? {
                    x if *x > 0 => *x -= 1,
                    _ => return Err(String::from("cannot decrement: value should stay between 0 and 1000")),
                }
            },
            'e' => {
                // Pushes the top item mod the 2nd item onto the stack.
                let top = memory.pop().ok_or("e error: stack is empty")?;
                let second = memory.pop().ok_or("e error: stack is empty")?;
                let result = top % second;
                memory.push(second)?;
                memory.push(top)?;
                memory.push(result)?;
            },
            'f' => {
                // Prints the top item on the stack as an ASCII character.
                let elem = memory.last().ok_or("f error: stack is empty")?;
                print!("{}", *elem as u8 as char);
            },
            'g' => {
                // Adds the first 2 stack items together and pushes the result to the stack.
                let top = memory.pop().ok_or("g error: stack is empty")?;
                let second = memory.pop().ok_or("g error: stack is empty")?;
                let result = second + top;
                memory.push(second)?;
                memory.push(top)?;
                memory.push(result)?;
            },
            'h' => {
                // Gets input from the user as a number and pushes to the stack.
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {},
                    Err(_x) => {
                        return Err(String::from("h error: readline failed"));
                    }
                };
                let trimmed = input.trim();
                match trimmed.parse::<i64>() {
                    Ok(i) => {
                        if i < 0 || i > 1000 {
                            return Err(String::from("h error: input is not an integer in the allowed range 0-1000"));
                        }
                        memory.push(i)?;
                    },
                    Err(_) => {
                        return Err(String::from("h error: input is not an integer"));
                    }
                };
            },
            'i' => {
                // Increments the top item of the stack by 1.
                match memory.last_mut().ok_or("i error: stack is empty")? {
                    x if *x < 1000 => *x += 1,
                    _ => return Err(String::from("cannot increment: value should stay between 0 and 1000")),
                }
            },
            'j' => {
                // Gets input from the user as a character and pushes that characters ASCII code onto the stack.
                let read_char = match std::io::stdin().bytes().next()
                    .ok_or("j error: cannot read a char from stdin")? {
                        Ok(x) => x,
                        Err(_) => {
                            return Err(String::from("j error: cannot get input"));
                        }
                    };
                memory.push(read_char as i64)?;
            },
            'l' => {
                // Swaps the 1st and 2nd items on the stack.
                let top = memory.pop().ok_or("l error: stack is empty")?;
                let second = memory.pop().ok_or("l error: stack is empty")?;
                memory.push(top)?;
                memory.push(second)?;
            },
            'm' => {
                // Multiplies the first 2 stack items together and pushes the result onto the stack.
                let top = memory.pop().ok_or("m error: stack is empty")?;
                let second = memory.pop().ok_or("m error: stack is empty")?;
                let result = second * top;
                memory.push(second)?;
                memory.push(top)?;
                memory.push(result)?;
            },
            'p' => {
                // Divides the top item on the stack by the 2nd item and pushes the result onto the stack.
                let top = memory.pop().ok_or("p error: stack is empty")?;
                let second = memory.pop().ok_or("p error: stack is empty")?;
                if second == 0 {
                    return Err(String::from("p error: dividing by zero"));
                }
                let result = top / second;
                memory.push(second)?;
                memory.push(top)?;
                memory.push(result)?;
            },
            'q' => {
                // Duplicates the top item on the stack.
                let elem = (*(memory.last().ok_or("q error: stack is empty")?)).clone();
                memory.push(elem)?;
            },
            'r' => {
                // Pushes the total length of the stack onto the stack.
                memory.push(memory.length() as i64)?;
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
