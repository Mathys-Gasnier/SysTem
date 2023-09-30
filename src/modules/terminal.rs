
pub enum TerminalBuilderError {
    MissingTerminalFlagForStartCommand(String) // When the TerminalBuilder.get_start_command_flag cannot find a flag
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
        let mut command = std::process::Command::new("cmd");
        command.args([ "/c", "start", &self.terminal ]);

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