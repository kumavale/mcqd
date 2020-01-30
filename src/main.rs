extern crate chrono;
use chrono::prelude::*;

use std::env;
use std::fs;
use std::process;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("invalid argument");
        eprintln!("Usage: cargo run -- [OPTION] <answer sheet> <answer key>");
        eprintln!("Options:");
        eprintln!("  -d,--date        Print date");
        eprintln!("  -n,--no-color    Turn colorization off always");
        process::exit(1);
    }

    let mut answers_sum     = 0;
    let mut correct_answers = 0;
    let mut no_color        = false;
    let mut print_date      = false;

    // parse options
    let mut args_i: usize = 1;
    while args_i as i32 <= args.len() as i32 - 3 {
        match &*args[args_i] {
            "-n" | "--no-color" => no_color = true,
            "-d" | "--date"     => print_date = true,
            _ => {
                eprintln!("invalid option: {}", args[args_i]);
                process::exit(1);
            }
        }
        args_i += 1;
    }

    let answer_sheet = fs::read_to_string(&args[args_i])?;
    let answer_key   = fs::read_to_string(&args[args_i+1])?;

    let (answer_sheet, s_len) = split(&answer_sheet);
    let (answer_key,   k_len) = split(&answer_key);

    if s_len != k_len {
        eprintln!("missing answers count");
        process::exit(1);
    }

    let max_nod = number_of_digits(s_len);
    let mut answer_key = answer_key.iter();

    if print_date {
        let dt = Utc::now();
        println!("[{}-{:>02}-{:>02} {:>02}:{:>02}] UTC",
            dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute());
    }

    if no_color {
        for (i, sheet) in (1..).zip(answer_sheet.iter()) {
            let key   = answer_key.next().unwrap();
            answers_sum += 1;

            if sheet == key {
                correct_answers += 1;
                print!("{}{}. {}\n", " ".repeat(max_nod-number_of_digits(i)), i, sheet);
            } else {
                print!("{}{}. {} : {}\n", " ".repeat(max_nod-number_of_digits(i)), i, sheet, key);
            }
        }
    } else {
        for (i, sheet) in (1..).zip(answer_sheet.iter()) {
            let key   = answer_key.next().unwrap();
            answers_sum += 1;

            if sheet == key {
                correct_answers += 1;
                print!("{}{}. \x1b[32m{}\x1b[0m\n", " ".repeat(max_nod-number_of_digits(i)), i, sheet);
            } else {
                print!("{}{}. \x1b[31m{} : {}\x1b[0m\n", " ".repeat(max_nod-number_of_digits(i)), i, sheet, key);
            }
        }
    }

    let percentage = correct_answers as f32 / answers_sum as f32 * 100_f32;
    println!("\n{}/{} ... {}%", correct_answers, answers_sum, percentage);

    Ok(())
}

fn split(s: &str) -> (Vec<String>, usize) {
    let mut vec = Vec::new();
    let mut len = 0;
    let mut block_comment_nest = 0;
    let s = s.replace("/*", " /* ").replace("*/", " */ ");

    'outer: for line in s.lines() {
        for token in line.split_whitespace() {
            // skip block comment  (/* ... */)
            if 0 < block_comment_nest {
                if token == "/*" {
                    block_comment_nest += 1;
                } else if token == "*/" {
                    block_comment_nest -= 1;
                }
                continue;
            }
            if token == "/*" {
                block_comment_nest += 1;
                continue;
            }
            // skip line comment  (//)
            if token.starts_with("//") {
                continue 'outer;
            }

            let token: Vec<&str> = token.split("//").collect();
            len += 1;
            vec.push(token[0].to_string());
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

