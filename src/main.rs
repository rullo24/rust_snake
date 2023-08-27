// ***************************************************************************************** \\
//                                                                                           \\

// Importing Crate Modules
mod cmd_print;
mod map;
mod input_fn;
mod movement;
mod data_structures;
mod linked;

use std::time::Duration;
use cmd_print::{starting_screen_print, printing_snake_map, printing_initial_snake_map, end_game_screen_print};
use map::{setup_snake_map, calculate_map_size};
use input_fn::{checking_for_enter_to_start, snake_movement_grab_char_wo_enter};
use data_structures::{GameSettings, ButtonSettings};
use std::collections::LinkedList;
use linked::linked_list_add_to_back;
use input_fn::reading_config_file;

//                                                                                           \\
// ***************************************************************************************** \\

fn main(){  

    let mut linked_list = LinkedList::new();

    // Creating base variable to hold game settings & current button press
    let mut base_game_settings = GameSettings::default();
    let mut button_settings = ButtonSettings::default();

    let config_file_vec: Vec<String> = reading_config_file();

    // Creates field size from user terminal arguments
    calculate_map_size(&config_file_vec, &mut base_game_settings);

    // Changes colours of background for initial print
    starting_screen_print();

    // Starts game once enter is pressed by clearing terminal
    checking_for_enter_to_start();

    // Create vector for snake game field.
    let mut snake_game_field: Vec<Vec<char>> = vec![vec![' '; (base_game_settings.field_size_col + 2) as usize]; (base_game_settings.field_size_row + 2) as usize];

    // Setting initial borders for snake map --> Ensuring random placement of snake & food
    setup_snake_map(&mut snake_game_field, &mut base_game_settings, &mut linked_list);

    // Adding head of linked list
    linked_list_add_to_back(&mut linked_list, base_game_settings.snake_starting_row, base_game_settings.snake_starting_col);
    
    // Printing out vector --> Can be reused
    printing_initial_snake_map(&snake_game_field);

    // Grabbing char without enter --> Playing the game
    snake_movement_grab_char_wo_enter(&mut button_settings, &mut snake_game_field, &mut base_game_settings, &mut linked_list); 

    // Printing final screen w/ score on it.
    end_game_screen_print(&base_game_settings);

    // Waiting 2 seconds to show final game over screen.
    std::thread::sleep(Duration::from_secs(2));
}