use crate::command::Command;

pub struct FailCommand {
    message: String,
}

impl FailCommand {
    pub fn new(message: String) -> FailCommand {
        FailCommand { message: message }
    }
}

impl Command for FailCommand {
    fn execute(&self) -> Result<String, String> {
        Err(self.message.clone())
    }
}
