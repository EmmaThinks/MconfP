// pub fn placeholder() -> Result<(), Box<dyn Error>> {
//    Ok(())
//}

use ddc_hi::{self, Display};
use std::error::Error;

// flags with no value
pub fn show_displays() -> Result<(), Box<dyn Error>> {
    print!("\nDETECTED MONITORS:\n\n");
    for ds in Display::enumerate() {
        let mod_name = ds
            .info
            .model_name
            .unwrap_or_else(|| "Unnamed Monitor".to_string());
        let id = ds.info.id.chars().last().ok_or("No id Found")?;

        print!("MONITOR:\nModel Name: {mod_name}\nId: {id}\n\n");
    }
    println!(
        "Note: To target specific monitors with --target: (-t), you can use either their id or model name\nIf you want to target an integrated monitor (eDP) you can use the 'int' keyword\n"
    );
    Ok(())
}

pub fn help() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// flags with value/target
pub fn set_brightness(targets: &Vec<String>, val: &u16) -> Result<(), Box<dyn Error>> {
    if targets.contains(&"all".to_string()) {}

    for mut dis in Display::enumerate() {
        dis.update_capabilities()?;
    }
    Ok(())
}

pub fn turn_off(targets: &Vec<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
