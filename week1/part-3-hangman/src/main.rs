// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;

use rand::Rng;
use std::fs;
use std::io;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn get_char_input() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error reading line.");
    guess = guess.replace("\n", "");
    return guess;
}


fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let mut secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)

    let mut guess_fail_times = 0;
    let mut guessed_word = Vec::new();
    let mut guessed_right_word = Vec::with_capacity(secret_word_chars.len());
    for _idx in 0..secret_word_chars.len() {
        guessed_right_word.push('-');
    }
    println!("Welcome to CS110L Hangman!\n");

    while guess_fail_times < NUM_INCORRECT_GUESSES {
        println!("=========================");
        println!("The word so far is {}", guessed_right_word.iter().collect::<String>());
        println!("You have guessed the following letters: {}", guessed_word.iter().collect::<String>());
        println!("Please guess a letter:");

        let guess = get_char_input();
        if guess.len() != 1 {
            println!("Don't cheat, input should be one character only");
            continue;
        } else {
            let guess_char = guess.chars().nth(0).unwrap();
            guessed_word.push(guess_char);

            // get input char
            let char_idx = secret_word_chars.iter().position(|&r| r == guess_char);
            if char_idx != None {
                // guess right word
                guessed_right_word[char_idx.unwrap()] = guess_char;
                // replace secret_word_chars char with -
                secret_word_chars[char_idx.unwrap()] = '-';
            } else {
                // guess wrong word
                guess_fail_times += 1;
            }
        }

        println!("You have {} guesses left", NUM_INCORRECT_GUESSES - guess_fail_times);
        if NUM_INCORRECT_GUESSES <= guess_fail_times {
            println!("Sorry, you ran out of guesses!")
        }
        let char_idx = guessed_right_word.iter().position(|&r| r == '-');
        if char_idx == None {
            println!("Congratulations you guessed the secret word: {}!", guessed_right_word.iter().collect::<String>());
            break;
        }
        println!("=========================\n");
    }
}
