use std::fs;
use std::error::Error;
use rand::prelude::IndexedRandom;

pub fn grab_word(file_path: &str) -> Result<String, Box<dyn Error>>{
    let contents = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = contents.lines().collect();

    match lines.choose(&mut rand::rng()) {
        Some(&line) => Ok(line.to_string()),
        None => Err("File contains no lines".into()),
    }
}

pub fn start_game(selected_word: String){
    unimplemented!();
}
