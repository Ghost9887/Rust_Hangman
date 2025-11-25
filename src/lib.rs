use std::fs;
use std::error::Error;
use rand::prelude::IndexedRandom;

struct GameData{
    lives: u8,
    selected_word: String,
    guessed_letters: Vec<char>,
}

impl GameData {
    fn new(selected_word: String) -> GameData{
        GameData{
            lives: 8,
            selected_word: selected_word,
            guessed_letters: vec![],
        }
    }
}

pub fn grab_word(file_path: &str) -> Result<String, Box<dyn Error>>{
    //read the file and put in a string
    let contents = fs::read_to_string(file_path)?;
    
    //seperate the lines and put them in a vector
    let lines: Vec<&str> = contents.lines().collect();

    //choose a random line and return if found
    match lines.choose(&mut rand::rng()) {
        Some(&line) => Ok(line.to_string()),
        None => Err("File contains no lines".into()),
    }
}

pub fn start_game(selected_word: String){
    let mut gd = GameData::new(selected_word);

    println!("Starting game...");
}

