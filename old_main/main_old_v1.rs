use ddc_hi::{Ddc, Display};
use std::{error::Error, io::Write, thread};

fn main() -> Result<(), Box<dyn Error>> {
    // REFACTORIZATION OF MY MCONFIG SCRIPT FOR MONITOR CONTROLS
    //
    // !!!IMPORTANT!!!
    // MAKE SURE YOUR USER IS ALLOWED IN THE i2c GROUP, OTHERWISE THIS SCRIPT WON'T WORK
    // this script is outdated, previous versions contain a binary file using this exact script.

    let mut args = std::env::args().skip(1);
    if args.len() == 0 {
        println!(
            "Make sure to provide arguments! use --help or -h after calling the script to see the available flags"
        );
        std::process::exit(1);
    }

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--show-displays" | "-sd" => {
                println!("Detected Monitors:");
                for mut dis in Display::enumerate() {
                    dis.update_capabilities()?;
                    println!(
                        "{}",
                        dis.info
                            .model_name
                            .unwrap_or_else(|| "Unnamed monitor".to_string())
                    );
                }
            }
            "--change-brightness" | "-cb" => {
                let bgr_amount = args
                    .next()
                    .ok_or("No argument given at --change-brightness / -cb")?
                    .parse::<u16>()?;
                let mut handles: Vec<thread::JoinHandle<Result<(), String>>> = Vec::new();

                for mut dis in Display::enumerate() {
                    let handle = thread::spawn(move || {
                        dis.update_capabilities()
                            .map_err(|e| format!("Unable to update capabilities: {e}"))?;

                        let name = dis.info.model_name.unwrap_or_else(|| "Unnamed".to_string());
                        dis.handle.set_vcp_feature(0x10, bgr_amount).map_err(|e| {
                            format!("Failed to change Monitor: {name} brightness, {e}")
                        })?;
                        println!("Changed Monitor {name} brightness to: {bgr_amount}");
                        Ok(())
                    });

                    handles.push(handle);
                }

                for h in handles {
                    match h.join() {
                        Ok(Ok(())) => {}
                        Ok(Err(e)) => eprintln!("Thread OK, Error inside thread: {e}"),
                        Err(_) => eprintln!("Thread panicked while trying to change brightness"),
                    };
                }
            }
            "--turn-off-monitors" | "-turnOff" => {
                println!("Shutting down Monitors");
                let mut handles: Vec<thread::JoinHandle<Result<(), String>>> = Vec::new();

                for mut dis in Display::enumerate() {
                    let handle = thread::spawn(move || -> Result<(), String> {
                        dis.update_capabilities().map_err(|e|
                            format!("Unable to Update capabilities when trying to turn off monitors: {e}")
                        )?;

                        let name = dis.info.model_name.unwrap_or_else(|| "Unnamed".to_string());
                        dis.handle.set_vcp_feature(0xD6, 0x04).map_err(|e| {
                            format!("Unable to turn off monitor: {name}, Error: {e}")
                        })?;
                        Ok(())
                    });

                    handles.push(handle);
                }

                for h in handles {
                    match h.join() {
                        Ok(Ok(())) => {}
                        Ok(Err(e)) => eprintln!("Thread OK, Error inside thread: {e}"),
                        Err(_) => {
                            eprintln!("Thread panicked while trying to turn off monitors");
                        }
                    };
                }
            }
            "--help" | "-h" => {
                print!(
                    "\nMconfP is a simple script for monitor management in terminal interfaces\n
    Flags:\n
        --show-displays | -sd                           Display the name of detected Monitors\n
        --change-brightness <value> | -cb <value>       Change the brightness of all detected Monitors\n
        --turn-off-monitors | -turnOff                  Turn off all detected Monitors\n
        --help | -h                                     Display this message\n

    ! IMPORTANT ! Before executing any command, make sure that your user is allowed in the i2c group, otherwise the script won't work\n\n"
                );

                std::io::stdout().flush()?;
            }
            _ => println!(
                "Provide valid arguments, use -h / --help to see the available flags for this script"
            ),
        }
    }

    Ok(())
}
