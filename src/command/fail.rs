use crate::command::Command;
use std::io::Write;

pub struct FailCommand {
    message: String,
}

impl Command for FailCommand {
    fn execute(&self, _out_writer: &mut dyn Write, err_writer: &mut dyn Write) -> Result<(), ()> {
        writeln!(err_writer, "{}", &self.message).unwrap();
        Err(())
    }
}

impl FailCommand {
    pub fn new(message: String) -> FailCommand {
        FailCommand { message: message }
    }
}
