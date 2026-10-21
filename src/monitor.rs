// pub fn placeholder() -> Result<(), Box<dyn Error>> {
//    Ok(())
//}

use ddc_hi::{self, Display};
use std::error::Error;
use std::fs::{read_dir};
use std::io::{self, Write};

// flags with no value
pub fn show_displays() -> Result<(), Box<dyn Error>> {
    print!("\nDETECTED MONITORS:\n\n");
    for ds in Display::enumerate() {
        let mod_name = ds
            .info
            .model_name
            .unwrap_or_else(|| "Unnamed Monitor".to_string());
        let id = ds.info.id.chars().last().ok_or("No id Found (probably an integrated monitor)")?;

        print!("MONITOR:\nModel Name: {mod_name}\nId: {id}\n\n");
    }
    println!(
        "Note: To target specific monitors with --target: (-t), you can use either their id or model name\nIf you want to target an integrated monitor (eDP) you can use the 'int' keyword (DO NOT USE ID TO TARGET INTEGRATED MONITORS)\n"
    );
    Ok(())
}

pub fn help() -> Result<(), Box<dyn Error>> {
    print!(
                    "\nMconfP is a simple script for monitor management in terminal interfaces\n
    Flags:\n
        --show-displays | -sd                           Display the name of detected Monitors\n
        --change-brightness:<value> | -cb:<value>       Change the brightness of detected Monitors\n
        --target:<monitor> | -t:<monitor>               Apply the configuration only to selected monitors\n
        --turn-off-monitors | -turnOff                  Turn off detected Monitors\n
        --help | -h                                     Display this message\n

    ! IMPORTANT ! Before executing any command, take in mind that you'll need sudo permissions while executing the script (using sudo before executing this script)\n\n"
                );

                std::io::stdout().flush()?;
    Ok(())
}

// flags with value/target
pub fn set_brightness(targets: &Vec<String>, val: &u16) -> Result<(), Box<dyn Error>> {
    // targeting integrated monitors
    if targets.contains(&"all".to_string())
        || targets.contains(&"int".to_string())
        || targets.is_empty()
    {
        let path = "/sys/class/backlight";
        let entry = read_dir(path)?
            .filter_map(|e| e.ok())
            .find(|e| e.path().is_dir())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No backlight device found"));

        let path = entry?.path().join("brightness");
        let mut file = std::fs::OpenOptions::new().write(true).open(path)?;
        let val_per = val * 192;
        file.write_all(&val_per.to_string().into_bytes())?;
    }

    // normal function to target external monitors (might need some optimizations)
    for mut dis in Display::enumerate() {
        match dis.update_capabilities() {
            Ok(_) => {
                
            },
            Err(_) => {
                break;
            },
        }; 
    }
    Ok(())
}

pub fn turn_off(targets: &Vec<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
