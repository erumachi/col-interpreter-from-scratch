mod stack;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;
use std::result::Result;
use crate::stack::Stack;
use std::io;
use std::io::Read;
use std::io::Write;

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

fn find_corresponding_u(prog_str: &Vec<char>, t_pos: usize) -> usize {
    let mut found_ts = 0;
    for (idx, ch) in prog_str.iter().skip(t_pos + 1).enumerate() {
        match ch {
            't' => {
                found_ts += 1;
            },
            'u' => {
                if found_ts == 0 {
                    return idx + t_pos + 1;
                }
                found_ts -= 1;
            },
            _ => {}
        }
    }
    prog_str.len() - 1
}   

fn find_corresponding_t(prog_str: &Vec<char>, u_pos: usize) -> usize {
    let mut found_us = 0;
    let u_pos_from_end = prog_str.len() - 1 - u_pos;
    for (idx, ch) in prog_str.iter().rev().skip(u_pos_from_end + 1).enumerate() {
        match ch {
            't' => {
                if found_us == 0 {
                    return u_pos - 1 - idx;
                }
                found_us -= 1;
            },
            'u' => {
                found_us += 1;
            },
            _ => {}
        }
    }
    0
}

fn prompt_user() -> Result<String, String> {
    print!("\ndebug> ");
    match io::stdout().flush() {
        Ok(_) => {},
        Err(x) => {
            return Err(format!("Error flushing: {}", x));
        }
    }
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_x) => {
            return Err(String::from("h error: readline failed"));
        }
    };
    Ok(input.trim().to_string())
}


// I know, I should refactor and break this stuff into smaller pieces,
// but it's not worth the effort, I probably won't touch this code again when I'm done :^)
fn interpret_program(memory: &mut Stack<i64>, program: &str, debug: bool) -> Result<(), String> {
    let prog_str: Vec<char> = program.chars().collect();
    let mut curr_prog_idx = 0;
    let mut breakpoint_enabled = debug;
    let mut breakpoints = Vec::new();
    while curr_prog_idx < prog_str.len() {
        let mut curr_instruction = prog_str[curr_prog_idx];
        let mut executing_cmd = false;
        if debug {
            let mut asking_user = true;
            if breakpoints.iter().any(|&i| i == curr_prog_idx) {
                breakpoint_enabled = true;
            }
            while asking_user && breakpoint_enabled {
                let cmd = prompt_user()?;
                match cmd.as_str() {
                    "help" => {
                        // print help : help
                        println!("Commands are:");
                        println!("pidx                print current program instruction index");
                        println!("pprog               print current program text");
                        println!("pstack              print current stack");
                        println!("pist                print current program instruction");
                        println!("step                execute the next instruction");
                        println!("cont                continue until next breakpoint/end of program");
                        println!("brk  <index>        insert a breakpoint at the given index");
                        println!("exec <character>    execute the instruction <character>");
                    },
                    "pidx" => {
                        // print current prog index : pidx
                        println!("Program index: {}", curr_prog_idx);
                    },
                    "pprog" => {
                        // print the whole prog : pprog
                        println!("Program: {}", program);
                    },
                    "pstack" => {
                        // print current stack : pstack
                        println!("Stack: {:?}", memory);
                    },
                    "pist" => {
                        // print current prog instruction : pist
                        println!("Stack: {}", curr_instruction);
                    },
                    "step" => {
                        // execute next instruction: step
                        asking_user = false;
                    },
                    "cont" => {
                        // run until completion/breakpoint : cont
                        asking_user = false;
                        breakpoint_enabled = false;
                    },
                    brk_cmd if brk_cmd.starts_with("brk ") => {
                        // breakpoint at given index (from 0): brk <index>
                        let brk_idx = match brk_cmd.split_ascii_whitespace().skip(1).next() {
                            Some(x) => x,
                            None => {
                                println!("Invalid brk command");
                                continue;
                            }
                        };
                        match brk_idx.parse::<usize>() {
                            Ok(i) => {
                                if i < program.len() {
                                    breakpoints.push(i);
                                    println!("Breakpoint set at {}", i);
                                } else {
                                    println!("Error: breakpoint out of range");
                                }
                            },
                            Err(_) => {
                                println!("Error: breakpoint should be a positive number");
                            }
                        };
                    },
                    exec_cmd if exec_cmd.starts_with("exec ") => {
                        // execute the given operator: exec <character>
                        let exec_chr = match exec_cmd.split_ascii_whitespace()
                            .skip(1).next().ok_or("Error: exec command without argument")?
                            .chars().next() {
                            Some(x) => x,
                            None => {
                                println!("Error: invalid exec command");
                                continue;
                            }
                        };
                        curr_instruction = exec_chr;
                        executing_cmd = true;
                        asking_user = false;
                    },
                    _ => {
                        println!("Invalid command");
                    },
                };
            }
        }
        match curr_instruction {
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
            'k' => {
                // Skips the next command if the top item on the stack is 0.
                let top = memory.last().ok_or("k error: stack is empty")?;
                if *top == 0 {
                    curr_prog_idx += 1;
                }
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
            'n' => {
                // If the 1st item on the stack is equal to the 2nd item, push a 1 to the stack, else push a 0.
                let top = memory.pop().ok_or("m error: stack is empty")?;
                let second = memory.pop().ok_or("m error: stack is empty")?;
                let value_to_push = if top == second { 1 } else { 0 };
                memory.push(second)?;
                memory.push(top)?;
                memory.push(value_to_push)?;
            },
            'o' => {
                // Pops the (top item on the stack)th item on the stack.
                // Note: nth element is from the top of the stack, not the bottom
                let idx = memory.last().ok_or("o error: stack is empty")?.clone() as usize;
                memory.remove(memory.length() - 1 - idx)?;
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
            's' => {
                // Swaps the 1st and (top item on the stack)th items on the stack.
                // Note: nth element is from the top of the stack, not the bottom
                let idx = memory.last().ok_or("s error: stack is empty")?.clone() as usize;
                memory.swap(memory.length()-1, memory.length() - 1 - idx)?;
            },
            't' => {
                // If the top item on the stack is 0, jumps to the corresponding ‘u’ in the program, otherwise does nothing.
                let top = memory.last().ok_or("t error: stack is empty")?;
                if *top == 0 {
                    let u_idx = find_corresponding_u(&prog_str, curr_prog_idx);
                    curr_prog_idx = u_idx;
                }
            },
            'u' => {
                // If the top item on the stack is not 0, jumps back to the corresponding ‘t’ in the program, otherwise does nothing.
                let top = memory.last().ok_or("u error: stack is empty")?;
                if *top != 0 {
                    let t_idx = find_corresponding_t(&prog_str, curr_prog_idx);
                    curr_prog_idx = t_idx;
                }
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
        if !(debug && executing_cmd) {
            curr_prog_idx += 1;
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print!("Usages:\n\t{} run <expression> [--debug]\n\t{} file <filename> [--debug]\n", args[0], args[0]);
        return;
    }
    let max_stack_size = 100;
    let mut memory: Stack<i64> = Stack::new(max_stack_size);
    let debug = args.len() > 3 && args[3] == "--debug";
    let program = match run_or_file(&args[1], &args[2]) {
        Some(x) => x,
        None => {
            println!("Error: operation failed");
            return;
        }
    };

    match interpret_program(&mut memory, &program, debug) {
        Ok(_) => {},
        Err(s) => {
            println!("\nExecution error: {}", s);
        }
    };
}
