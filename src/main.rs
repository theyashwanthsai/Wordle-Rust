use colored::*;
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::HashSet;

const WORDS: &str = include_str!("words.txt"); 
const LEN: usize = 5;
const MAXCHANCES: usize = 6;

fn worldlist() -> Vec<String> {
    WORDS
        .split('\n')
        .skip(2)
        .filter(|line| line.len() == LEN)
        .collect()
}

struct Game
{
    available_words: Vec<String>,
    word: String,
    guess_letters: HashSet<char>,
    guesses: Vec<String>,
}

impl Game
 {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let available_words = worldlist();
        let word = rng.random_slice_entry(&available_words).unwrap().clone();
        Self{
            available_words,
            word,
            guess_letters: HashSet::new(),
            guesses: Vec::new(),
        }
    }

    fn colorcode(&mut self) {
        self.guesses.iter().enumerate().for_each(|(guessnumber, guess)|{
            print!("{}:", guessnumber+1);
            guess.chars().enumerate().for_each(|(pos, c)|{
                let display = if self.word.chars().nth(pos).unwrap() == c {
                    format!("{c}").green()
                }
                else if self.word.chars().any(|wc| wc == c) {
                    format!("{c}").yellow()
                }
                else {
                    format!("{c}").red()
                };
                print!("{display}");
            });
            println!();
        })
    }

    fn display_invalid_letter(&self){
        if !self.guess_letters.is_empty() {
            print!("Letters not present in the word!: ");
            self.guess_letters.iter()
                .for_each(|letter| print!("{letter} "));
            println!();
        }
    }

    fn promt_guess(&mut self) -> String{
        print!("{}", format!("Enter your guess ({} letters), and press Enter", LEN));
        self.display_invalid_letter();
        let mut guess  = String::new();
        let mut valid_guess = false;
        while !valid_guess{
            guess = String::new();
            std::io::stdin().read_line(&mut guess).unwrap();
            if guess.len() != LEN {
                print!("{}", format!("Your guessed word must be of {} lenght", LEN).red());
            }
            else{
                self.guesses.push(guess.clone());
                valid_guess = true;
            }
        }
        guess
    }

    fn gameover(&self, guess: &str) -> bool {
        let ntries = self.guesses.len();
        if guess == self.word{
            print!("YAY!!, you guessed the word in {} tries", ntries);
            true
        }
        else if ntries > MAXCHANCES{
            print!("{}", format!("You ran out of chances, the word was {}", self.word).red());
            true
        }
        else{
            false
        }
    }
}

fn main(){
    let mut game = Game::new();
    loop{
        game.colorcode();
        let guess = game.promt_guess();
        if game.gameover(&guess){
            break;
        }
    }
}
