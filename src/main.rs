use std::collections::HashMap;
use std::{env, fs};

struct Token {
    id: String,
    value: String,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        return Token {
            id: self.id.clone(),
            value: self.value.clone()
        };
    }
}

fn scan(pattern: &str, chars: &mut Vec<char>, idx: &mut i32, len: i32) -> String {
    let mut cur = *(*chars).get(*idx as usize).unwrap();
    let mut ret: String = "".to_string();

    while pattern.contains(cur) && *idx < len {
        ret.push(cur);
        *idx += 1;
        if *idx < len {
            cur = *(*chars).get(*idx as usize).unwrap();
        }
    }
    return ret;
}

fn tokenize(code: String) -> Vec<Token> {
    let mut chars: Vec<char> = code.chars().collect();
    let mut ret: Vec<Token> = Vec::new();
    let mut idx: i32 = 0;
    let len: i32 = code.len() as i32;

    while idx < len as i32 {
        let cur: char = *chars.get(idx as usize).unwrap();

        if "1235467890".contains(cur) {
            ret.push(
                Token {
                    id: String::from("number"),
                    value: scan("1234567890", &mut chars, &mut idx, len)
                }
            );
        } else if "abcdefghijklmnopqrstuvwxyz_".contains(cur) {
            ret.push(
                Token {
                    id: String::from("symbol"),
                    value: scan("abcdefghijklmnopqrstuvwxyz_", &mut chars, &mut idx, len)
                }
            );
        } else if "+-*/&|^>=<".contains(cur) {
            ret.push(
                Token {
                    id: String::from("operator"),
                    value: scan("+-*/&|^><", &mut chars, &mut idx, len)
                }
            );
            idx += 1;
        } else if "{}?!".contains(cur) {
            ret.push(
                Token {
                    id: String::from(cur),
                    value: String::from(cur)
                }
            );
            idx += 1;
        } else if " \n\t\r".contains(cur) {
            idx += 1;
        } else if cur == '\'' {
            let char = *chars.get((idx + 1) as usize).unwrap();

            ret.push(
                Token {
                    id: String::from("number"),
                    value: (char as u8).to_string()
                }
            );
            idx += 3;
        } else {
            eprintln!("Unexpected char {cur}");
            idx += 1;
        }
    }

    return ret;
}

fn scan_block(tokens: &Vec<Token>, idx: &mut i32) -> Vec<Token> {
    let mut ret: Vec<Token> = Vec::new();
    let mut level: i32 = 1;

    while level > 0 {
        let cur = (*tokens.get(*idx as usize).unwrap()).clone();
        *idx += 1;
        //println!("[type={}, value={}]", cur.id, cur.value);

        if cur.id.as_str() == "{" {
            level += 1;
        } else if (cur).id.as_str() == "}" {
            level -= 1;

            if level == 0 {
                return ret;
            }
        }
        ret.push(cur);
    }
    return ret;
}

fn run(tokens: &Vec<Token>, stack: &mut Vec<i32>, functions: &mut HashMap<String, Vec<Token>>, memory: &mut [i32; 65536], running: &mut bool) {
    let mut idx: i32 = 0;
    let len: i32 = tokens.len() as i32;

    while *running && idx < len {
        let ref cur = *(*tokens).get(idx as usize).unwrap();
        idx += 1;

        match (*cur).id.as_str() {
            "number" => {
                (*stack).push((*cur).value.as_str().parse::<i32>().unwrap());
            }
            "symbol" => {
                match (*cur).value.as_str() {
                    "not" => {
                        let value: i32 = (*stack).pop().unwrap();
                        (*stack).push(if value > 0 {0} else {1});
                    }
                    "dup" => {
                        (*stack).push(*(*stack).get((*stack).len() - 1).unwrap());
                    }
                    "swap" => {
                        let b: i32 = (*stack).pop().unwrap();
                        let a: i32 = (*stack).pop().unwrap();
                        (*stack).push(b);
                        (*stack).push(a);
                    }
                    "rot" => {
                        let c: i32 = (*stack).pop().unwrap();
                        let b: i32 = (*stack).pop().unwrap();
                        let a: i32 = (*stack).pop().unwrap();
                        (*stack).push(c);
                        (*stack).push(b);
                        (*stack).push(a);
                    }
                    "print" => {
                        print!("{}", (*stack).pop().unwrap());
                    }
                    "emit" => {
                        print!("{}", char::from_u32((*stack).pop().unwrap() as u32).unwrap());
                    }
                    "cr" => {
                        println!();
                    }
                    "def" => {
                        let name = (*(*tokens).get(idx as usize).unwrap()).value.clone();
                        idx += 2;
                        let block = scan_block(tokens, &mut idx);
                        functions.insert(name.clone(), block);
                    }
                    _ => {
                        if !functions.contains_key((*cur).value.as_str()) {
                            *running = false;
                            println!("Unknown symbol {}", (*cur).value.as_str());
                            return;
                        }
                        let clone_functions = functions.clone();
                        run(clone_functions.get((*cur).value.as_str()).unwrap(), stack, functions, memory, running);
                    }
                }
            }
            "operator" => {
                let b: i32 = (*stack).pop().unwrap();
                let a: i32 = (*stack).pop().unwrap();

                match (*cur).value.as_str() {
                    "+" => {(*stack).push(a + b);}
                    "-" => {(*stack).push(a - b);}
                    "*" => {(*stack).push(a * b);}
                    "/" => {(*stack).push(a / b);}
                    "&" => {(*stack).push(a & b);}
                    "|" => {(*stack).push(a | b);}
                    "^" => {(*stack).push(a ^ b);}
                    "<<" => {(*stack).push(a << b);}
                    ">>" => {(*stack).push(a >> b);}
                    ">" => {(*stack).push(if a > b {1} else {0});}
                    "<" => {(*stack).push(if a < b {1} else {0});}
                    ">=" => {(*stack).push(if a >= b {1} else {0});}
                    "<=" => {(*stack).push(if a <= b {1} else {0});}
                    "==" => {(*stack).push(if a == b {1} else {0});}
                    "&&" => {(*stack).push(if (a > 0) && (b > 0) {1} else {0});}
                    "||" => {(*stack).push(if (a > 0) || (b > 0) {1} else {0});}
                    _ => {
                        *running = false;
                        eprintln!("Unknown operator {}", (*cur).value.as_str());
                    }
                }
            }
            "?" => {
                let addr: usize = (*stack).pop().unwrap() as usize;
                (*stack).push((*memory)[addr]);
            }
            "!" => {
                let value: i32 = (*stack).pop().unwrap();
                let addr: usize = (*stack).pop().unwrap() as usize;
                (*memory)[addr] = value;
            }
            _ => {
                *running = false;
                eprintln!("Unexpected {}", (*cur).id.as_str())
            }
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        eprintln!("1 argument expected (input file), got {} instead.", args.len() - 1);
        return;
    }

    let filepath = (*args.get(1).unwrap()).clone();

    let code: String = fs::read_to_string(filepath).expect("Failed to read input file");
    let tokens: Vec<Token> = tokenize(code);

    let mut stack: Vec<i32> = Vec::new();
    let mut functions: HashMap<String, Vec<Token>> = HashMap::new();
    let mut memory: [i32; 65536] = [0; 65536];
    let mut running: bool = true;
    run(&tokens, &mut stack, &mut functions, &mut memory, &mut running);
}
