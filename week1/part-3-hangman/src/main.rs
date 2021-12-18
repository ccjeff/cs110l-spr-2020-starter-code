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
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn get_char() -> String {
    print!("Please guess a letter: ");
    // Make sure the prompt from the previous line gets displayed:
    let mut guess = String::new();
    loop {
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        if guess.len() == 2 {
            break;
        } else {
            println!("wrong input length: {}, try again.", guess.len());
            guess = String::new();
        }
    }
    guess
}

// returns the matching index(s) for the character in string.
/*
@param: inputted char, matching target
@return: Vec<u32> containing occurance index
*/
fn check_match(word: &String, target: &String)->Vec<u32> {
    let mut index_vec: Vec<u32> = Vec::new();
    for i in 0..word.len() {
        if word.chars().nth(i).unwrap() == target.chars().nth(0).unwrap() {
            index_vec.push(i as u32);
        }
    }
    index_vec
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut display_word: Vec<char> = Vec::new();
    for _ in 0..secret_word.len() {
        display_word.push('_');
    }
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    let mut round = 0;
    let mut correct_guess = 0;
    while (round < NUM_INCORRECT_GUESSES) && (correct_guess < secret_word.len()) {
        let guessed = get_char();
        let matchpts = check_match(&secret_word, &guessed);
        if matchpts.len() == 0 {
            println!("Sorry, no match for {}", guessed);
            round += 1;
        } else {
            correct_guess += 1;
            println!("Match found for {}", guessed);
            for i in matchpts {
                display_word[i as usize] = secret_word_chars[i as usize];
            }
        }
        println!("the current guess is: {}", display_word.iter().collect::<String>());
    }
    if correct_guess == secret_word.len() {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
