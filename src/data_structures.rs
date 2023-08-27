#[derive(Debug)]
pub struct GameSettings{
    pub field_size_row: i8,
    pub field_size_col: i8,
    pub snake_starting_row: i8,
    pub snake_starting_col: i8,
    pub snake_prev_row: i8,
    pub snake_prev_col: i8,
    pub food_starting_row: i8,
    pub food_starting_col: i8,
    pub refresh_time_ms: u64,
    pub game_end: bool,
    pub game_score: i16
}

impl Default for GameSettings{
    fn default() -> Self {
        GameSettings {
            field_size_row: 10,
            field_size_col: 10,
            snake_starting_row: 1,
            snake_starting_col: 1,
            snake_prev_row: 1,
            snake_prev_col: 1,
            food_starting_row: 1,
            food_starting_col: 1,
            refresh_time_ms: 350,
            game_end: false,
            game_score: 0
        }
    }
}

pub struct ButtonSettings{
    pub current_char: char,
    pub print_char: char
}

impl Default for ButtonSettings{
    fn default() -> Self {
        ButtonSettings {
            current_char: '0',
            print_char: '^'
        }
    }
}