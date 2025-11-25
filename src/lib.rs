use std::fs;
use std::error::Error;
use rand::prelude::IndexedRandom;
use std::io;

struct GameData{
    lives: u8,
    temp_word: String,
    selected_word: String,
    guessed_letters: Vec<char>,
}

impl GameData {
    fn new(selected_word: String) -> GameData{
        GameData{
            lives: 8,
            temp_word: "*".repeat(selected_word.len()),
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
    println!("-------------------");
    println!("Welcome to Hang Man");

    //main loop
    loop{
        println!("Lives: {}", gd.lives);
        println!("Letters already guess: {:?}", gd.guessed_letters);
        println!("Type a letter into the terminal");
        println!("Word: ");
        println!("{}", gd.temp_word);
        println!("");
        let user_input = match get_user_input(&mut gd) {
            Ok(c) => {
                gd.guessed_letters.push(c);
                c
            },
            Err(e) => {
                println!("{e}\n");
                continue;
            },
        };

        let found = check_letters(&user_input, &gd.selected_word, &mut gd.temp_word);

        if !found {
            gd.lives -= 1;
        }

        if gd.lives < 1 {
            println!("You have lost!");
            println!("The word was: {}", gd.selected_word);
            break;
        }
        else if gd.selected_word == gd.temp_word {
            println!("You have won!");
            break;
        }

        continue;
    }
}

fn get_user_input(gd: &mut GameData) ->  Result<char, String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|_| "Failed to read line".to_string())?;

    if let Some(first_char) = input.to_lowercase().trim().chars().next() {
        if gd.guessed_letters.contains(&first_char) {
            Err("Already guessed this letter try again".to_string())
        }
        else if first_char.is_alphabetic() {
            Ok(first_char)
        }else{
            Err("Invalid input".to_string())
        }
    } else{
        Err("No input provided".to_string())
    }

}

fn check_letters(letter: &char, word: &String, temp_word: &mut String) -> bool{
    let mut found = false;

    for(i, ch) in word.chars().enumerate() {
        if ch == *letter {
            temp_word.replace_range(i..=i, &letter.to_string());
            found = true;
        }
    }
    found
}

