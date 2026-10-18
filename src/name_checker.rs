use ddc_hi;
use std::error::Error;

pub fn fetch_name(names: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ddc_names: Vec<String> = vec![];
    if names.is_empty() || names.contains(&"all".to_string()) {
        return Ok(vec![]);
    }
    if names.contains(&"int".to_string()) {
        ddc_names.push("int".to_string());
    }

    for display in ddc_hi::Display::enumerate() {
        let mod_name = display
            .info
            .model_name
            .unwrap_or_else(|| "Unnamed Monitor".to_string());
        let id = display.info.id.chars().last().ok_or("No id Found")?;

        if names.contains(&mod_name) || names.contains(&id.to_string()) {
            ddc_names.push(id.to_string());
        }
    }

    Ok(ddc_names)
}
