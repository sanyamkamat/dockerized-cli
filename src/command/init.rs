use crate::command::Command;
use std::io::Write;
use std::fs::{create_dir, write};
use std::path::PathBuf;
use indoc::indoc;

pub struct InitCommand {
    // TODO should this struct hold a &Path instead? How about the lifetime?
    working_dir: PathBuf,
}

fn break_with_errors<T,E: std::fmt::Display> (result: &Result<T,E>, err_writer: &mut dyn Write) -> Result<(),()> {
    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            writeln!(err_writer, "{}", err).unwrap();
            Err(())
        }
    }
}

impl Command for InitCommand {
    // TODO should that be Result<&str, &str>?
    fn execute(&self, out_writer: &mut dyn Write, err_writer: &mut dyn Write) -> Result<(),()> {
        let result = create_dir(self.working_dir.join(".dockerized"));
        break_with_errors(&friendlify_already_exists_error(result), err_writer)?;

        let result = self.write_dockerfile();
        break_with_errors(&result, err_writer)?;

        writeln!(out_writer, "Initialized .dockerized").unwrap();
        Ok(())
    }
}

impl InitCommand {
    pub fn new(working_dir: PathBuf) -> InitCommand {
        InitCommand {
            working_dir: working_dir,
        }
    }

    fn write_dockerfile(&self) -> Result<(),std::io::Error> {
        let dockerfile_path = self.working_dir.join(".dockerized").join("Dockerfile.dockerized");
        write(dockerfile_path, indoc!("
            FROM busybox

            # Add your build dependencies here
        "))?;
        
        Ok(())
    }
}

// TODO is there a more idiomatic way to convert one error type to another?
fn friendlify_already_exists_error(result: std::io::Result<()>) -> Result<(),String> {
    if let Err(err) = result {
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => Err("Already initialized".to_string()),
            _ => Err(err.to_string()),
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
    use std::io::sink;

    #[test]
    fn creates_directory() {
        let working_dir = tempdir().unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        init_command.execute(&mut sink(), &mut sink()).unwrap();

        assert!(working_dir.path().join(".dockerized").is_dir());
    }

    #[test]
    fn output_on_success() {
        let mut out_buffer = Vec::new();
        let mut out_writer = std::io::Cursor::new(&mut out_buffer);
        let mut err_buffer = Vec::new();
        let mut err_writer = std::io::Cursor::new(&mut err_buffer);
        let working_dir = tempdir().unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        init_command.execute(&mut out_writer, &mut err_writer).unwrap();

        assert_eq!(String::from_utf8(out_buffer).unwrap(), "Initialized .dockerized\n");
        assert_eq!(String::from_utf8(err_buffer).unwrap(), "");
    }

    #[test]
    fn fails_if_already_initialized() {
        let mut err_buffer = Vec::new();
        let mut err_writer = std::io::Cursor::new(&mut err_buffer);
        let working_dir = tempdir().unwrap();
        create_dir(&working_dir.path().join(".dockerized")).unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        init_command.execute(&mut sink(), &mut err_writer).unwrap_err();

        assert_eq!(String::from_utf8(err_buffer).unwrap(), "Already initialized\n");
    }

    #[test]
    fn creates_dockerfile() {
        let working_dir = tempdir().unwrap();

        let init_command = InitCommand::new(working_dir.path().to_owned());
        init_command.execute(&mut sink(), &mut sink()).unwrap();

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
