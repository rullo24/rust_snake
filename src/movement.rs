use std::collections::LinkedList;

use crate::{ data_structures::{ButtonSettings, GameSettings}, linked::linked_list_add_to_back};
use rand::Rng;

pub fn check_direction_and_move_snake(ref_button_settings: &mut ButtonSettings, ref_snake_game_field: &mut Vec<Vec<char>>, ref_base_game_settings: &mut GameSettings, ref_linked_list: &mut LinkedList<(i8, i8)>){

    // Moving Up
    if ref_button_settings.current_char == 'W'{
        // [row][col]
        ref_snake_game_field[((*ref_base_game_settings).snake_starting_row) as usize][((*ref_base_game_settings).snake_starting_col) as usize] = ' ';
        (*ref_base_game_settings).snake_prev_row = (*ref_base_game_settings).snake_starting_row.clone();
        (*ref_base_game_settings).snake_starting_row -= 1;
        ref_snake_game_field[((*ref_base_game_settings).snake_starting_row) as usize][((*ref_base_game_settings).snake_starting_col) as usize] = '^';

        (*ref_button_settings).print_char = '^';
    }
    // Moving Down
    else if ref_button_settings.current_char == 'S'{
        // [row][col]
        ref_snake_game_field[((*ref_base_game_settings).snake_starting_row) as usize][((*ref_base_game_settings).snake_starting_col) as usize] = ' ';
        (*ref_base_game_settings).snake_prev_row = (*ref_base_game_settings).snake_starting_row.clone();
        (*ref_base_game_settings).snake_starting_row += 1;
        ref_snake_game_field[((*ref_base_game_settings).snake_starting_row) as usize][((*ref_base_game_settings).snake_starting_col) as usize] = 'v';

        (*ref_button_settings).print_char = 'v';

    }
    // Moving Left
    else if ref_button_settings.current_char == 'A'{
        // [row][col]
        ref_snake_game_field[((*ref_base_game_settings).snake_starting_row) as usize][((*ref_base_game_settings).snake_starting_col) as usize] = ' ';
        (*ref_base_game_settings).snake_prev_col = (*ref_base_game_settings).snake_starting_col.clone();
        (*ref_base_game_settings).snake_starting_col -= 1;
        ref_snake_game_field[((*ref_base_game_settings).snake_starting_row) as usize][((*ref_base_game_settings).snake_starting_col) as usize] = '<';

        (*ref_button_settings).print_char = '<';

    }
    // Moving Right
    else if ref_button_settings.current_char == 'D'{
        // [row][col]
        ref_snake_game_field[((*ref_base_game_settings).snake_starting_row) as usize][((*ref_base_game_settings).snake_starting_col) as usize] = ' ';
        (*ref_base_game_settings).snake_prev_col = (*ref_base_game_settings).snake_starting_col.clone();
        (*ref_base_game_settings).snake_starting_col += 1;
        ref_snake_game_field[((*ref_base_game_settings).snake_starting_row) as usize][((*ref_base_game_settings).snake_starting_col) as usize] = '>';

        (*ref_button_settings).print_char = '>';

    }

    // Removing the original head --> Saving to variable.
    let deleted_head = (*ref_linked_list).pop_front().unwrap();

    // Replacing head with a more current head.
    ref_linked_list.push_front((
        (*ref_base_game_settings).snake_starting_row,
        (*ref_base_game_settings).snake_starting_col,
    ));

    // Shuffling the linked list by moving data down one node
    let mut prev_data_to_copy = deleted_head;
    let mut buffer_data_to_copy = prev_data_to_copy.clone();
    let mut counter_initial_done: bool = false;
    let mut counter: i8 = 0;

    for (snake_x, snake_y) in ref_linked_list.iter_mut() {
        if counter != 0 {
            if !counter_initial_done {
                buffer_data_to_copy.0 = (*snake_x).clone();
                buffer_data_to_copy.1 = (*snake_y).clone();
    
                std::mem::swap(&mut prev_data_to_copy.0, snake_x);
                std::mem::swap(&mut prev_data_to_copy.1, snake_y);
    
                counter_initial_done = true;
            }
            else {
                prev_data_to_copy = buffer_data_to_copy.clone();
    
                buffer_data_to_copy.0 = (*snake_x).clone();
                buffer_data_to_copy.1 = (*snake_y).clone();
    
                std::mem::swap(&mut prev_data_to_copy.0, snake_x);
                std::mem::swap(&mut prev_data_to_copy.1, snake_y);
            }        
        }

        counter += 1; 
    }
}

pub fn check_food_taken_by_snake(ref_snake_game_field: &mut Vec<Vec<char>>, ref_base_game_settings: &mut GameSettings, ref_linked_list: &mut LinkedList<(i8, i8)>){

    // Checking if the snake head is ontop of the snake food
    if ((*ref_base_game_settings).snake_starting_row) as usize == ((*ref_base_game_settings).food_starting_row) as usize &&
    ((*ref_base_game_settings).snake_starting_col) as usize == ((*ref_base_game_settings).food_starting_col) as usize{
        
        random_new_food_location(ref_snake_game_field, ref_base_game_settings, ref_linked_list);

        (*ref_base_game_settings).game_score += 1;

        // Initially adding linked list node with coordinates out of bounds --> These are changed in next iteration of printing map
        linked_list_add_to_back(ref_linked_list, -1, -1);

    }
}

pub fn random_new_food_location(ref_snake_game_field: &mut Vec<Vec<char>>, ref_base_game_settings: &mut GameSettings, ref_linked_list: &mut LinkedList<(i8, i8)>){
    while (*ref_linked_list).contains(&((*ref_base_game_settings).food_starting_row, (*ref_base_game_settings).food_starting_col)){
        ref_base_game_settings.food_starting_row = rand::thread_rng().gen_range(1..=ref_base_game_settings.field_size_row);
        ref_base_game_settings.food_starting_col = rand::thread_rng().gen_range(1..=ref_base_game_settings.field_size_col);
    }
    
    // (ref_base_game_settings.food_starting_row, ref_base_game_settings.food_starting_col) == 
    // (ref_base_game_settings.snake_starting_row, ref_base_game_settings.snake_starting_col) 

    let _ = ref_snake_game_field[((*ref_base_game_settings).food_starting_row) as usize][((*ref_base_game_settings).food_starting_col) as usize] = 'x';
}

pub fn check_head_sitting_on_body(ref_base_game_settings: &mut GameSettings, ref_linked_list: &mut LinkedList<(i8, i8)>){
    let mut cloned_linked_list = (*ref_linked_list).clone();

    cloned_linked_list.pop_front();

    let snake_head_position_tuple = ((*ref_base_game_settings).snake_starting_row, (*ref_base_game_settings).snake_starting_col);

    if cloned_linked_list.contains(&snake_head_position_tuple){
        (*ref_base_game_settings).game_end = true;
    }
    std::thread::sleep(std::time::Duration::from_micros(500));
}