pub trait Command {
    fn execute(&self) -> Result<String, String>;
}

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

pub struct FailCommand {
    message: String,
}
impl FailCommand {
    pub fn new(message: String) -> FailCommand {
        FailCommand{
            message: message.to_string()
        }
    }
}
impl Command for FailCommand {
    fn execute(&self) -> Result<String, String> {
        Err(self.message.clone())
    }
}