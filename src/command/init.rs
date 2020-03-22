use crate::command::Command;
use std::path::PathBuf;
use std::fs::create_dir;

pub struct InitCommand {
    working_dir: PathBuf
}

impl InitCommand {
    pub fn new(working_dir: PathBuf) -> InitCommand {
        InitCommand{
            working_dir: working_dir
        }
    }
}

impl Command for InitCommand {
    fn execute(&self) -> Result<String, String> {
        create_dir(self.working_dir.join(".dockerized")).unwrap();

        Ok("Initialized .dockerized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    use std::env::temp_dir;
    use std::path::Path;

    fn given_working_dir_is_empty(working_dir: &Path) {
        if working_dir.exists() {
            remove_dir_all(&working_dir).unwrap();
        }
        create_dir(&working_dir).unwrap();
    }

    #[test]
    fn creates_directory() {
        let working_dir = temp_dir().join("dockerized-test");
        given_working_dir_is_empty(&working_dir);

        let init_command = InitCommand::new(working_dir.to_owned());
        let result = init_command.execute();

        assert_eq!(result.unwrap(), "Initialized .dockerized");
        assert!(working_dir.join(".dockerized").is_dir());
    }
}