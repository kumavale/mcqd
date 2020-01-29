use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("invalid argument");
        eprintln!("Usage:\n    cargo run <answer sheet> <answer key>");
        std::process::exit(1);
    }

    let answer_sheet = fs::read_to_string(&args[1])?;
    let answer_key   = fs::read_to_string(&args[2])?;

    let (answer_sheet, s_len) = split(&answer_sheet);
    let (answer_key,   k_len) = split(&answer_key);
    let mut answers_sum = 0;
    let mut correct_answers = 0;

    if s_len != k_len {
        eprintln!("missing answers count");
        std::process::exit(1);
    }

    let max_nod = number_of_digits(s_len);
    let mut answer_key = answer_key.iter();

    for (i, sheet) in answer_sheet.iter().enumerate() {
        let key   = answer_key.next().unwrap();
        answers_sum += 1;

        if sheet == key {
            correct_answers += 1;
            println!("{}{}. \x1b[32m{}\x1b[0m", " ".repeat(max_nod-number_of_digits(i+1)), i+1, sheet);
        } else {
            println!("{}{}. \x1b[31m{} : {}\x1b[0m", " ".repeat(max_nod-number_of_digits(i+1)), i+1, sheet, key);
        }
    }

    let percentage = correct_answers as f32 / answers_sum as f32 * 100_f32;
    println!("\n{}/{} ... {}%", correct_answers, answers_sum, percentage);

    Ok(())
}

fn split(s: &str) -> (Vec<String>, usize) {
    let mut vec = Vec::new();
    let mut len = 0;
    let mut block_comment_now = false;
    let s = s.replace("/*", " /* ").replace("*/", " */ ");

    'outer: for line in s.lines() {
        for token in line.split_whitespace() {
            // skip block comment  (/* ... */)
            if block_comment_now {
                if token == "*/" {
                    block_comment_now = false;
                }
                continue;
            } else if token == "/*" {
                block_comment_now = true;
                continue;
            }
            // skip line comment  (//)
            if token.starts_with("//") {
                continue 'outer;
            }

            len += 1;
            vec.push(token.to_string());
            break;
        }
    }

    (vec, len)
}

fn number_of_digits(mut num: usize) -> usize {
    let mut nod = 0;

    while num != 0 {
        nod += 1;
        num /= 10;
    }

    nod
}

