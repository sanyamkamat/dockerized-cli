use crate::command::Command;
use std::fs::create_dir;
use std::path::PathBuf;

pub struct InitCommand {
    // TODO should this struct hold a &Path instead? How about the lifetime?
    working_dir: PathBuf,
}

impl InitCommand {
    pub fn new(working_dir: PathBuf) -> InitCommand {
        InitCommand {
            working_dir: working_dir,
        }
    }
}

impl Command for InitCommand {

    // TODO should that be Result<&str, &str>?
    fn execute(&self) -> Result<String, String> {
        let result = create_dir(self.working_dir.join(".dockerized"));
        friendlify_already_exists_error(result)?;

        Ok("Initialized .dockerized".to_string())
    }
}

// TODO is there a more idiomatic way to convert one error type to another?
fn friendlify_already_exists_error(result: std::io::Result<()>) -> Result<(), String> {
    if let Err(err) = result {
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => return Err("Already initialized".to_string()),
            _ => return Err(err.to_string()),
        }
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn creates_directory() {
        let working_dir = tempdir().unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        let result = init_command.execute();

        assert_eq!(result.unwrap(), "Initialized .dockerized");
        assert!(working_dir.path().join(".dockerized").is_dir());
    }

    #[test]
    fn fails_if_already_initialized() {
        let working_dir = tempdir().unwrap();
        create_dir(&working_dir.path().join(".dockerized")).unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        let result = init_command.execute();

        assert_eq!(result.unwrap_err(), "Already initialized");
    }
}
