use std::io::Write;

pub trait Command {
    fn execute(&self, out_writer: &mut dyn Write, err_writer: &mut dyn Write) -> Result<(), ()>;
}

mod init;
mod fail;

pub use init::InitCommand;
pub use fail::FailCommand;
