use crate::command::Command;
use std::fs::create_dir;
use std::path::PathBuf;
use indoc::indoc;

pub struct InitCommand {
    // TODO should this struct hold a &Path instead? How about the lifetime?
    working_dir: PathBuf,
}

impl Command for InitCommand {
    // TODO should that be Result<&str, &str>?
    fn execute(&self) -> Result<String, String> {
        let result = create_dir(self.working_dir.join(".dockerized"));
        friendlify_already_exists_error(result)?;

        self.write_dockerfile()?;
        Ok("Initialized .dockerized".to_string())
    }
}

impl InitCommand {
    pub fn new(working_dir: PathBuf) -> InitCommand {
        InitCommand {
            working_dir: working_dir,
        }
    }

    fn write_dockerfile(&self) -> Result<(), String> {
        let dockerfile_path = self.working_dir.join(".dockerized").join("Dockerfile.dockerized");
        let result = std::fs::write(dockerfile_path, indoc!("
            FROM busybox

            # Add your build dependencies here
        "));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
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
        init_command.execute().unwrap();

        assert!(working_dir.path().join(".dockerized").is_dir());
    }

    #[test]
    fn output_on_success() {
        let working_dir = tempdir().unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        let result = init_command.execute();

        assert_eq!(result.unwrap(), "Initialized .dockerized");
    }

    #[test]
    fn fails_if_already_initialized() {
        let working_dir = tempdir().unwrap();
        create_dir(&working_dir.path().join(".dockerized")).unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        let result = init_command.execute();

        assert_eq!(result.unwrap_err(), "Already initialized");
    }

    #[test]
    fn creates_dockerfile() {
        let working_dir = tempdir().unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        init_command.execute().unwrap();

        let dockerfile_path = working_dir
            .path()
            .join(".dockerized")
            .join("Dockerfile.dockerized");

        assert!(dockerfile_path.is_file());

        let dockerfile_content = std::fs::read_to_string(dockerfile_path).unwrap();
        assert_eq!(dockerfile_content, indoc!("
            FROM busybox

            # Add your build dependencies here
        "));
    }
}
