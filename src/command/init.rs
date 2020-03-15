use crate::command::Command;

pub struct InitCommand;

impl InitCommand {
    pub fn new() -> InitCommand {
        InitCommand
    }
}

impl Command for InitCommand {
    fn execute(&self) -> Result<String, String> {
        Err("TODO".to_string())
    }
}
