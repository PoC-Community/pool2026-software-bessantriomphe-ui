use std::fs;
use std::error::Error;
use crate::models::Task;
pub fn load_json() -> Result<Vec<Task>, Box<dyn Error>>{
    let message: String = fs::read_to_string("src/data/tasks.json")?;
    let messager:Vec<Task>=serde_json::from_str(&message)?;
    Ok(messager)
}

pub fn save_json(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>>{
    let oklm: String=serde_json::to_string_pretty(&tasks)?;
    fs::write("src/data/tasks.json", oklm)?;
    Ok(())
}