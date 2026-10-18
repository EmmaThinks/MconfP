// pub fn placeholder() -> Result<(), Box<dyn Error>> {
//    Ok(())
//}

use std::error::Error;

// flags with no value
pub fn show_displays() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn help() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// flags with value/target
pub fn set_brightness(targets: &Vec<String>, val: u16) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn turn_off(targets: &Vec<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
