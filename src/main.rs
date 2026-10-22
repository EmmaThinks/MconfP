use std::{
    env::{Args, args}, iter::Skip, thread,
    sync::Arc,
};

use crate::{
    monitor::{help, set_brightness, show_displays, turn_off},
    name_checker::fetch_name,
};
mod monitor;
mod name_checker;

pub struct Config {
    pub targets: Vec<String>,
    pub commands: Vec<Command>,
}

pub enum Command {
    ShowDisplays,
    SetBrightness(u16),
    TurnOff,
    Help,
    Err(String),
}

fn parse_args(args: &mut Skip<Args>) -> Config {
    let mut flags: Vec<Command> = vec![];
    let mut tars: Vec<String> = vec![];

    for arg in args {
        if let Some((flag_name, value)) = arg.split_once(':') {
            match flag_name {
                "--change-brightness" | "-cb" => match value.parse::<u16>() {
                    Ok(v) => flags.push(Command::SetBrightness(v)),
                    Err(e) => flags.push(Command::Err(e.to_string())),
                },
                "--target" | "-t" => {
                    tars.extend(value.split(',').map(String::from));
                }
                _ => {
                    let err_msg = format!(
                        "'{flag_name}' appears to be an unknown flag, consider using: --help to see the full list of flags"
                    );
                    flags.push(Command::Err(err_msg))
                }
            }
        } else {
            match arg.as_str() {
                "--show-displays" | "-sd" => flags.push(Command::ShowDisplays),
                "--turn-off-monitors" | "-turnOff" => flags.push(Command::TurnOff),
                "--help" | "-h" => flags.push(Command::Help),
                _ => flags.push(Command::Err(String::from(
                    "flag not found, consider using --help to see available",
                ))),
            };
        }
    }

    Config {
        targets: tars,
        commands: flags,
    }
}

fn exec_commands(conf: Config) -> Result<(), Box<dyn std::error::Error>> {
    
    let displays = fetch_name(&conf.targets)?;
    let shared_displays: Arc<[String]> = Arc::from(displays);
    println!("{shared_displays:#?}");

    let mut handles: Vec<thread::JoinHandle<Result<(), String>>> = Vec::new();

    for cmd in &conf.commands {
        match cmd {
            Command::ShowDisplays => show_displays()?,
            Command::Help => help()?,
            Command::TurnOff => {
                let cloned_arc = Arc::clone(&shared_displays);
                let handle = thread::spawn(move || {
                    turn_off(&cloned_arc).map_err(|e| {
                        format!("Error: {e}")
                    })?;
                    Ok(())
                });
                handles.push(handle);
            },
            Command::SetBrightness(val) => {
                let cloned_arc = Arc::clone(&shared_displays);
                let val = val.clone();
                let handle = thread::spawn(move || {
                    set_brightness(&cloned_arc, &val).map_err(|e| {
                        format!("Error: {e}")
                    })?;
                    Ok(())
                });
                handles.push(handle);
            },
            Command::Err(e) => eprintln!("Argument Error: {e}"),
        }
    }

    for h in handles {
        match h.join() {
            Ok(Ok(_)) => {},
            Ok(Err(e)) => eprintln!("Thread OK, Error inside thread: {e}"),
            Err(e) => eprintln!("Thread panicked: {e:?}")
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args().skip(1);
    /* if args.len() == 0 {
        println!(
            "Make sure to provide arguments! use --help or -h after calling the script to see the available flags"
        );
        std::process::exit(1);
    }*/

    exec_commands(parse_args(&mut args))?;
    Ok(())
}
