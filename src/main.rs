extern crate dockerized;
extern crate clap;

use dockerized::command::{Command, InitCommand, FailCommand};
use clap::{App, SubCommand, Arg};

fn main() {
    let command = parse_cli_arguments().unwrap();
    let result = command.execute(&mut std::io::stdout(), &mut std::io::stderr());
    match result {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }
}

fn parse_cli_arguments() -> Result<Box<dyn Command>, String> {
    let matches = App::new("dockerized")
        .version("0.10.0")
        .subcommand(
            SubCommand::with_name("init").about("Initializes a new .dockerized directory"),
        )
        .subcommand(
            SubCommand::with_name("fail").about("Fail").arg(
                Arg::with_name("message").required(true)
            )
        )
        .get_matches();

    match matches.subcommand() {
        ("init", _) => Ok(Box::new(InitCommand::new(std::env::current_dir().unwrap()))),
        ("fail", Some(subm)) => Ok(Box::new(FailCommand::new(subm.value_of("message").unwrap().to_string()))),
        (command, _) => Err(format!("Unexpected command: {}", command))
    }
}
