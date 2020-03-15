pub trait Command {
    fn execute(&self) -> Result<String, String>;
}

mod init;
mod fail;

pub use init::InitCommand;
pub use fail::FailCommand;
