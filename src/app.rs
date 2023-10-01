use std::thread;

use evalexpr::*;
use raylib::prelude::*;

use crate::modules::{gitkey::GitKey, terminal::{TerminalBuilder, TerminalBuilderError}};

// Command prefixes
enum Prefix {
    NONE,    // No prefix
    CALC,    // Calculator prefix
    Command, // System Command prefix
    GitKey   // Get gitkey from gitkeys prefix
}

// Find the prefix from the input
fn find_prefix(input: &str) -> Prefix {
    
    if input.starts_with("=") {
        return Prefix::CALC;
    }

    if input.starts_with(">") {
        return Prefix::Command;
    }

    if input.starts_with("$") {
        return Prefix::GitKey
    }

    Prefix::NONE
}

// The result of executing a command
enum ExecuteResult {
    Output(String),    // Changes the text to the output string
    Clipboard(String), // Changes the clipboard to the string and exit the app
    Exit               // Exits the app
}

// Executes a command and return it's result
fn execute(input: &str) -> ExecuteResult {

    let prefix = find_prefix(input);

    match prefix {
        Prefix::CALC => {
            // Get the command without the "=", use evalexpr to eval it and returns the result
            let spliced_string = &String::from(input)[1..];
            ExecuteResult::Output(format!("={}", eval(spliced_string).unwrap()))
        },
        Prefix::Command => {
            // Get the command without the ">" prefix
            let spliced_string = &String::from(input)[1..];
            // Build a terminal to open the cmd and add the start command
            let mut terminal = TerminalBuilder::new("cmd");
            terminal.start_command(spliced_string);
            // Handle error that can occur when starting a terminal
            if let Some(error) = terminal.start() {
                match error {
                    TerminalBuilderError::MissingTerminalFlagForStartCommand(terminal) => {
                        return ExecuteResult::Output(format!("Cannot find {} start command flag.", terminal));
                    },
                    TerminalBuilderError::MissingConfigTerminalNew => {
                        return ExecuteResult::Output(String::from("Cannot find terminal.new in the config"));
                    }
                }
            }
            ExecuteResult::Exit
        },
        Prefix::GitKey => {
            // Gets the key without the "$", try to get the gitkey if it is found return it to the clipboard else return the error
            let spliced_string = String::from(&String::from(input)[1..]);
            if let Some(key) = GitKey::get(if spliced_string.trim() == "" { None } else { Some(spliced_string.clone()) }) {
                ExecuteResult::Clipboard(key)
            } else {
                ExecuteResult::Output(format!("Gitkey \"{}\" not found", spliced_string))
            }
        },
        Prefix::NONE => {
            // If not prefix was found but the command is "wsl" start a wsl terminal
            if input == "wsl" {
                TerminalBuilder::new("wsl").start();
                return ExecuteResult::Exit;
            }

            // Else send back an Error
            ExecuteResult::Output(String::from("No command found"))
        }
    }
}

pub fn start_app() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        // Creates the raylib window
        let (mut rl, thread) = raylib::init()
            .size(650, 65) // Width, Height
            .title("SysTem") // Window Title
            .undecorated() // No top bar with minimize, maximize and close button
            .build(); // Builds the window
        
        let current_monitor = raylib::core::window::get_current_monitor();
        let screen_size = Vector2::new(
            (raylib::core::window::get_monitor_width(current_monitor) / 3) as f32,
            (raylib::core::window::get_monitor_height(current_monitor) / 16) as f32
        );
        rl.set_window_size(screen_size.x as i32, screen_size.y as i32);
        
        // Stores the current input
        let mut input = String::from("");
        // Cursor position offset from the end (0 = end of input, length of input = start of line)
        let mut cursor: usize = 0;
        // Result mode is set to true after execution was called
        let mut result_mode = false;

        while !rl.window_should_close() {

            // Update
            // --------------------

            // While there is a char pressed
            while let Some(key) = rl.get_char_pressed() {
                // If it's less or equal to 0 it means that there isn't any new char pressed so we break
                if (key as u32) <= 0 {
                    break;
                }
                // Ensure the char is within acceptable char range
                if (key as u32) < 35 || (key as u32) > 125 {
                    continue;
                }
                // Add the char to the cursor position
                input.insert(input.len() - cursor, key);
                result_mode = false; // Disable result mode
            }

            // Add a space at the cursor if the spacebar is pressed
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
                input.insert(input.len() - cursor, ' ');
            }
        
            // Input spliced by the cursor (alias input from 0 to cursor position)
            let spliced_input = &input.clone()[..input.len() - cursor];

            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_BACKSPACE) {
                // If backspace is pressed while not in result mode remove a char at cursor position else empty the input
                if !result_mode && spliced_input.len() > 0 {
                    input.remove(input.len() - cursor - 1);
                } else if result_mode {
                    input = String::from("");
                }
            }
            
            // If the left arrow key is pressed and that we can still go left move the cursor left by one and switch off result mode
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT) {
                if cursor < input.len() {
                    cursor += 1;
                    result_mode = false;
                }
            }
    
            // If the right arrow key is pressed and that we can still go right move the cursor right by one and switch off result mode
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT) {
                if cursor > 0 {
                    cursor -= 1;
                    result_mode = false;
                }
            }
        
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER) {
                // If Enter is pressed in result mode we close the app
                if result_mode == true {
                    break;
                }
                // Execute the input, match and handle it's response
                let execute_result = execute(&input);
                match execute_result {
                    ExecuteResult::Output(output) => input = output,
                    ExecuteResult::Clipboard(clipboard) => {
                        rl.set_clipboard_text(&clipboard).unwrap();
                        break;
                    },
                    ExecuteResult::Exit => break
                };
                // Switch on result mode and reset cursor
                result_mode = true;
                cursor = 0;
            }
            
            // CTRL+C and CTRL+V behavior
            if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT_CONTROL) {
                if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_C) {
                    rl.set_clipboard_text(&input).unwrap();
                }
                if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_V) {
                    input = rl.get_clipboard_text().unwrap();
                }
            }

            // Gets the font size from the screen size
            let font_size = screen_size.y as i32 / 2;

            // Offset of the top left corner of the text from the top of the screen
            let text_offset = screen_size.y as i32 / 2 - font_size / 2;

            // Gets the position at which the cursor should be displayed on the app
            let cursor_position = raylib::text::measure_text(spliced_input, font_size);

            // Draw
            // --------------------
            let mut draw = rl.begin_drawing(&thread);

            draw.clear_background(raylib::color::rcolor(25, 25, 25, 1));

            // Draw the input
            draw.draw_text(&input, 15, text_offset, font_size, raylib::color::Color::WHITE);

            // Draw the cursor at the correct offset
            draw.draw_rectangle(15 + cursor_position, text_offset + font_size - 10, font_size / 2, 5, raylib::color::Color::WHITE);

        }
    })
}