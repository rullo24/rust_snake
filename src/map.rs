use rand::Rng;
use std::collections::LinkedList;
use crate::{GameSettings, ButtonSettings, movement::random_new_food_location};

pub fn calculate_map_size(ref_config_file_vec: &Vec<String>, ref_base_game_settings: &mut GameSettings){

    let row_size_string: String = (*ref_config_file_vec)[0].clone();
    let col_size_string: String = (*ref_config_file_vec)[1].clone();
    let refresh_time_ms_string: String = (*ref_config_file_vec)[2].clone();

    if let Some(row_size_string_equal_sign_pos) = row_size_string.find('=') {
        // Get the substring after the equal sign
        let value_row_size = &row_size_string[row_size_string_equal_sign_pos + 1..];

        // Parse the substring into an i32
        if let Ok(parsed_value) = value_row_size.parse::<i8>() {
            (*ref_base_game_settings).field_size_row = parsed_value;
        };
    }
    if let Some(col_size_string_equal_sign_pos) = col_size_string.find('=') {
        // Get the substring after the equal sign
        let value_col_size = &col_size_string[col_size_string_equal_sign_pos + 1..];

        // Parse the substring into an i32
        if let Ok(parsed_value) = value_col_size.parse::<i8>() {
            (*ref_base_game_settings).field_size_col = parsed_value;
        };
    }
    if let Some(refresh_time_string_equal_sign_pos) = refresh_time_ms_string.find('=') {
        // Get the substring after the equal sign
        let value_refresh_time = &refresh_time_ms_string[refresh_time_string_equal_sign_pos + 1..];

        // Parse the substring into an i32
        if let Ok(parsed_value) = value_refresh_time.parse::<u64>() {
            (*ref_base_game_settings).refresh_time_ms = parsed_value;
        };
    }
}

pub fn setup_snake_map(ref_snake_game_field: &mut Vec<Vec<char>>, ref_base_game_settings: &mut GameSettings, ref_linked_list: &mut LinkedList<(i8, i8)>){
    // Randomising starting position of snake & food --> Only within playable area
    ref_base_game_settings.snake_starting_row = rand::thread_rng().gen_range(1..=ref_base_game_settings.field_size_row);
    ref_base_game_settings.snake_starting_col = rand::thread_rng().gen_range(1..=ref_base_game_settings.field_size_col);
    ref_base_game_settings.snake_prev_row = ref_base_game_settings.snake_starting_row.clone();
    ref_base_game_settings.snake_prev_col = ref_base_game_settings.snake_starting_col.clone();

    ref_base_game_settings.food_starting_row = rand::thread_rng().gen_range(1..=ref_base_game_settings.field_size_row);
    ref_base_game_settings.food_starting_col = rand::thread_rng().gen_range(1..=ref_base_game_settings.field_size_col);

    // Ensuring that the food and snake start @ different positions
    random_new_food_location(ref_snake_game_field, ref_base_game_settings, ref_linked_list);

    for (row_index, row) in (*ref_snake_game_field).iter_mut().enumerate() {
        for (col_index, cell) in row.iter_mut().enumerate() {
            if row_index == 0 || col_index == 0 || row_index == ((*ref_base_game_settings).field_size_row + 1) as usize || col_index == ((*ref_base_game_settings).field_size_col + 1) as usize {
                *cell = '*';
            }
            else if row_index == ((*ref_base_game_settings).snake_starting_row) as usize && col_index == ((*ref_base_game_settings).snake_starting_col) as usize{
                *cell = '^';
            }
            else if row_index == ((*ref_base_game_settings).food_starting_row) as usize && col_index == ((*ref_base_game_settings).food_starting_col) as usize{
                *cell = 'x';
            }
        }
    }
}

pub fn cont_map_fix(ref_snake_game_field: &mut Vec<Vec<char>>, ref_base_game_settings: &mut GameSettings, ref_linked_list: &mut LinkedList<(i8, i8)>, ref_button_settings: &mut ButtonSettings){
    for (row_index, row) in (*ref_snake_game_field).iter_mut().enumerate() {
        for (col_index, cell) in row.iter_mut().enumerate() {
            if row_index == 0 || col_index == 0 || row_index == ((*ref_base_game_settings).field_size_row + 1) as usize || col_index == ((*ref_base_game_settings).field_size_col + 1) as usize {
                *cell = '*';
            }
            else if row_index == ((*ref_base_game_settings).snake_starting_row) as usize && col_index == ((*ref_base_game_settings).snake_starting_col) as usize{        
                *cell = (*ref_button_settings).print_char;
            }
            else if row_index == ((*ref_base_game_settings).food_starting_row) as usize && col_index == ((*ref_base_game_settings).food_starting_col) as usize{
                *cell = 'x';
            }
            else if (*ref_linked_list).contains(&(row_index as i8, col_index as i8)){
                *cell = '+';
            }
            else{
                *cell = ' ';
            }
        }
    }
}