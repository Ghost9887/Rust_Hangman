use std::fs;
use std::error::Error;

pub fn grab_word(file_path: &str) -> Result<String, Box<dyn Error>>{
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

pub fn start_game(selected_word: String){
    unimplemented!();
}
