use std::io;
use std::collections::HashSet;

struct Game {
    secret_word: String,
    secret_word_set: HashSet<char>,
    guesses: HashSet<char>,
    lives: i32,
    is_completed: bool,
}

impl Game {
    fn new() -> Self {
        let generated_word: String = generate_secret_word();
        let generated_word_set: HashSet<char> = generated_word.chars().collect();
        Self {
            secret_word: generated_word,
            secret_word_set: generated_word_set,
            guesses: HashSet::new(),
            lives: 6,
            is_completed: false,
        }
    }

    fn check_guess(&self, guess: &char) -> Option<bool> {
        if self.guesses.contains(&guess) {
            None
        } else {
            if self.secret_word_set.contains(&guess) {
                Some(true)
            } else {
                Some(false)
            }
        }
    }

    fn display_state(&self) {
        println!("Lives: {}", self.lives);
        println!("Guesses: {:?}", self.guesses);
        let partial_word: String = self.secret_word.chars().map(|c| {
            if self.guesses.contains(&c) {
                c
            } else {
                '_'
            }
        }).collect();
        println!("Can you guess the word? : {}", partial_word);
    }
}

fn generate_secret_word() -> String {
    String::from("hello")
    // "hello".to_owned()
}

fn read_char() -> Option<char> {
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).ok()?;
    
    match user_input.trim_end().chars().next() {
        Some(c) => Some(c),
        None => None,
    }
}

fn handle_guess_result(game: &mut Game, letter: char, guess_result: Option<bool>) {
    if let Some(is_correct) = guess_result {
        game.guesses.insert(letter);
        if is_correct {
            println!("Correct!");
            if game.secret_word_set.is_subset(&game.guesses) {
                game.is_completed = true;
                println!("Bob wins!");
            }
        } else {
            game.lives -= 1;
            println!("Incorrect!");
            if game.lives < 1 {
                game.is_completed = true;
                println!("Bob loses!");
            }
        }
    } else {
        println!("You already guessed that!");
    }
    game.display_state();
}

fn handle_user_input(game: &mut Game, bob_guess: Option<char>) {
    if let Some(letter) = bob_guess {
        let guess_result: Option<bool> = game.check_guess(&letter);
        handle_guess_result(game, letter, guess_result);
    } else {
        println!("Please enter a character!");
    }
}

fn main() {
    let mut game: Game = Game::new();

    while !game.is_completed {
        let bob_guess: Option<char> = read_char();
        handle_user_input(&mut game, bob_guess);
    }
    println!("Game over! The secret word was '{}'", game.secret_word)
}
