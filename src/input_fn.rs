use std::time::Duration;
use std::collections::LinkedList;
use std::fs::File;
use std::io::Read; 

use crate::{cmd_print::clear_terminal, printing_snake_map, 
    data_structures::{GameSettings, ButtonSettings}, 
    movement::{check_direction_and_move_snake, check_food_taken_by_snake, check_head_sitting_on_body}, 
    map::cont_map_fix};
use device_query::{DeviceQuery, DeviceState, Keycode};
///////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn reading_config_file() -> Vec<String>{
    let mut config_file: File;

    match File::open("./rust_snake_config.txt") {
        Ok(_) => {
            config_file = File::open("./rust_snake_config.txt").unwrap();
        }
        Err(_) => {
            config_file = File::open("../../rust_snake_config.txt").unwrap();
        }
    }
    let mut config_file_contents = String::new();
    config_file.read_to_string(&mut config_file_contents).unwrap();
    let config_file_vec_andstr: Vec<&str> = config_file_contents.split("\n").map(|line| line.trim()).filter(|line| line.contains('=')).collect();
    let config_file_vec_string: Vec<String> = config_file_vec_andstr.iter().map(|s| s.to_string()).collect();

    return config_file_vec_string
}

pub fn checking_for_enter_to_start(){
    let mut start_game_enter_to_be_pressed: String = "_".to_string();
    while start_game_enter_to_be_pressed != "" {
        start_game_enter_to_be_pressed.clear();
        let _ = std::io::stdin().read_line(&mut start_game_enter_to_be_pressed).unwrap();
        start_game_enter_to_be_pressed = start_game_enter_to_be_pressed.trim().to_string();
    }
    clear_terminal();
}

pub fn snake_movement_grab_char_wo_enter(ref_button_settings: &mut ButtonSettings, ref_snake_game_field: &mut Vec<Vec<char>>, ref_base_game_settings: &mut GameSettings, ref_linked_list: &mut LinkedList<(i8, i8)>) {
    let device_state: DeviceState = DeviceState::new();
    let mut time_since_last_key_down = std::time::Instant::now();
    let mut time_since_last_print = std::time::Instant::now();
    let mut key_as_string: String;
    let mut key_as_char_vector: Vec<char>;
    let possible_chars: Vec<char> = vec!['W', 'A', 'S', 'D'];

    while (*ref_base_game_settings).game_end == false {

        // Get a vector of all the keys that are currently pressed.
        let vec_all_keys_currently_pressed: Vec<Keycode> = device_state.get_keys();

        for key in vec_all_keys_currently_pressed.iter() {
            let time_value_now = std::time::Instant::now();

            if time_value_now.duration_since(time_since_last_key_down) > std::time::Duration::from_millis(100) {
                time_since_last_key_down = time_value_now;

                key_as_string = Keycode::to_string(key);
                key_as_char_vector = key_as_string.chars().collect();
                let key_as_char: char = key_as_char_vector[0];
                
                if possible_chars.contains(&key_as_char) {
                    (*ref_button_settings).current_char = key_as_char;      
                    // println!("{} | {}", key_as_char, (*ref_button_settings).current_char);
                }
            }
        }

        // Reprinting map & mapping snake direction every section of time (as specified by config file)
        if (std::time::Instant::now() - time_since_last_print) > Duration::from_millis((*ref_base_game_settings).refresh_time_ms){
            time_since_last_print = std::time::Instant::now();

            check_direction_and_move_snake(ref_button_settings, ref_snake_game_field, ref_base_game_settings, ref_linked_list);

            check_head_sitting_on_body(ref_base_game_settings, ref_linked_list);

            check_food_taken_by_snake(ref_snake_game_field, ref_base_game_settings, ref_linked_list);

            cont_map_fix(ref_snake_game_field, ref_base_game_settings, ref_linked_list, ref_button_settings);

            printing_snake_map(ref_snake_game_field);
        }

        // Checking if snake head within playing field --> Else GAME OVER
        if (*ref_base_game_settings).snake_starting_row == 0 || (*ref_base_game_settings).snake_starting_col == 0 ||
        (*ref_base_game_settings).snake_starting_row == (*ref_base_game_settings).field_size_row + 1 || 
        (*ref_base_game_settings).snake_starting_col == (*ref_base_game_settings).field_size_col + 1 {
            
            (*ref_base_game_settings).game_end = true; // Ending game if snake out of bounds
        }
    }
}
