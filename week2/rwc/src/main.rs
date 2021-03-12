use std::{env, io};
use std::process;
use std::fs::File;
use std::io::BufRead;

fn wc(filename: &String) -> Result<(u32, u32, u32), io::Error> {
    let mut line_number: u32 = 0;
    let mut word_number: u32 = 0;
    let mut char_number: u32 = 0;
    let file = File::open(filename)?;
    let mut last_str = String::from("");
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        for item in line_str.chars() {
            if item == ' ' {
                if last_str.len() > 0 {
                    word_number += 1;
                    last_str = String::from("");
                }
            } else {
                last_str.push(item);
            }
        }
        if last_str.len() > 0 {
            word_number += 1;
            last_str = String::from("");
        }
        line_number += 1;
        char_number += line_str.len() as u32 + 1;
    }
    Ok((line_number, word_number, char_number))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let res = wc(filename);
    if res.is_ok() {
        println!("{:?}, {}", res.unwrap(), filename);
    } else {
        panic!("file path: {} not exists!", filename);
    }
}
