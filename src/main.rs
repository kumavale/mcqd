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

    let answer_sheet = answer_sheet.split_whitespace();
    let mut answer_key = answer_key.split_whitespace();
    let mut answers_sum = 0;
    let mut correct_answers = 0;

    for (i, sheet) in answer_sheet.enumerate() {
        let key   = answer_key.next().unwrap();
        answers_sum += 1;

        if sheet == key {
            correct_answers += 1;
            println!("{}. \x1b[32m{}\x1b[0m", i+1, sheet);
        } else {
            println!("{}. \x1b[31m{} : {}\x1b[0m", i+1, sheet, key);
        }
    }

    let percentage = correct_answers as f32 / answers_sum as f32 * 100_f32;
    println!("\n{}/{} ... {}%", correct_answers, answers_sum, percentage);

    Ok(())
}

