use ddc_hi::{Ddc, Display};
use std::{error::Error, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    // REFACTORIZATION OF MY MCONFIG SCRIPT FOR MONITOR CONTROLS
    //
    // !!!IMPORTANT!!!
    // MAKE SURE YOUR USER IS ALLOWED IN THE i2c GROUP, OTHERWISE THIS SCRIPT WON'T WORK
    //

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
                for dis in &mut Display::enumerate() {
                    dis.update_capabilities()?;
                    println!(
                        "{}",
                        dis.info.model_name.as_deref().unwrap_or("Unnamed monitor")
                    );
                }
            }
            "--change-brightness" | "-cb" => {
                let bgr_amount = args
                    .next()
                    .ok_or("No argument given at --change-brightness / -cb")?
                    .parse::<u16>()?;

                for dis in &mut Display::enumerate() {
                    dis.update_capabilities()?;
                    dis.handle.set_vcp_feature(0x10, bgr_amount)?;
                    println!(
                        "Monitor {:?}: Brightness set to: {}",
                        dis.info.model_name.as_deref().unwrap_or("Unnamed monitor"),
                        bgr_amount
                    );
                }
            }
            "--turn-off-monitors" | "-turnOff" => {
                println!("Shutting down Monitors");
                for dis in &mut Display::enumerate() {
                    dis.update_capabilities()?;
                    dis.handle.set_vcp_feature(0xD6, 0x04)?
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
