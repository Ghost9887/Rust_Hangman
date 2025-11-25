use hangman::{grab_word};
use std::process;

fn main() {

    let file_path = String::from("words.txt");

    let selected_word = grab_word(&file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: '{err}'");
        process::exit(1);
    });

    println!("{selected_word}");

    //start_game(selected_word);
}


