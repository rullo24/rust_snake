use std::io::Write;
use std::io;
use std::time::Duration;
use crate::data_structures::GameSettings;

pub fn starting_screen_print(){
    // Creating Starting Screen
    println!("======================================================\n");
    print!("\x1B[33m");
    println!("*** Welcome to Snake (Written in Rust) ***\n");
    print!("\x1B[35m");

    println!("\tInstructions:");
    println!("\tMove Up == 'w'     ||   Head == '^'");
    println!("\tMove Down == 's'   ||   Head == 'v'");
    println!("\tMove Left == 'a'   ||   Head == '<'");
    println!("\tMove Right == 'd'  ||   Head == '>'");

    println!("\n\tFood == 'x'");

    print!("\x1B[36m");
    println!("\n### Please Press the Enter Key to Start the Game ###");
    print!("\x1B[0m");
    println!("\n======================================================");
}

pub fn end_game_screen_print(ref_base_game_settings: &GameSettings){

    let time_for_750_wait = std::time::Instant::now();
    while (std::time::Instant::now() - time_for_750_wait) < Duration::from_millis(750){
        // Do nothing --> Doesn't stop program
    }
    clear_terminal();

    match (*ref_base_game_settings).game_score{
        _ => {
            println!("======================================================\n");
            print!("\x1B[35m");
            println!("\t*** GAME OVER. ***\n");
            println!("\t### SCORE = {} ###\n", (*ref_base_game_settings).game_score);
            print!("\x1B[0m");
            println!("======================================================");
        }
    }  
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top-left corner
    io::stdout().flush().unwrap();
}

pub fn printing_initial_snake_map(ref_snake_game_field: &Vec<Vec<char>>){
    // Sleeping for snake movement & loading status
    let time_for_250_wait = std::time::Instant::now();
    while (std::time::Instant::now() - time_for_250_wait) > Duration::from_millis(250){
        // Do nothing --> Doesn't cause issues within program
    }

    for row in ref_snake_game_field {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn printing_snake_map(ref_snake_game_field: &mut Vec<Vec<char>>){
    clear_terminal();

    for row in ref_snake_game_field {
        for cell in row {
            print!("{}", *cell);
        }
        print!("\n");
    }
}