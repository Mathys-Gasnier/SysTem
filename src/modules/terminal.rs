use crate::config::get_config;


pub enum TerminalBuilderError {
    MissingTerminalFlagForStartCommand(String), // When the TerminalBuilder.get_start_command_flag cannot find a flag
    MissingConfigTerminalNew                    // When the config is missing the terminal.new value
}

pub struct TerminalBuilder {
    terminal: String,             // The name of the terminal to start
    start_command: Option<String> // The start command (command executed at the terminal start)
}

impl TerminalBuilder {
    
    // Create a TerminalBuilder from terminal name
    pub fn new(terminal: &str) -> Self {
        Self { terminal: String::from(terminal), start_command: None }
    }

    // Gets the flag to add a start command to a terminal
    fn get_start_command_flag(&self) -> Option<&str> {
        match self.terminal.as_str() {
            "cmd" => Some("/k"),
            _ => None
        }
    }

    pub fn start_command(&mut self, start_command: &str) {
        self.start_command = Some(String::from(start_command));
    }

    pub fn start(&self) -> Option<TerminalBuilderError> {

        // First create the command that starts the new terminal
        let config = get_config();
        if config.terminal.new.len() <= 0 {
            return Some(TerminalBuilderError::MissingConfigTerminalNew);
        }
        let mut command = std::process::Command::new(&config.terminal.new[0]);
        command.args(config.terminal.new[1..].iter().map(|a| a.replace("%c", &self.terminal)));

        // Then handle the start command by also getting the flag if needed
        if let Some(start_command) = &self.start_command {
            if let Some(flag) = self.get_start_command_flag() {
                command.args([ flag, start_command ]);
            } else {
                return Some(TerminalBuilderError::MissingTerminalFlagForStartCommand(self.terminal.clone()));
            }
        };
        
        // Start the terminal and return no error
        command.spawn().unwrap();

        None
    }

}