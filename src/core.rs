pub enum Command {
    Init,
    Fail(String)
}

pub trait Execute {
    fn execute(&self) -> Result<String, String>;
}

impl Execute for Command {
    fn execute(&self) -> Result<String, String> {
        match self {
            Command::Init => Ok("init".to_string()),
            Command::Fail(message) => Err(message.clone()),
        }
    }
}
