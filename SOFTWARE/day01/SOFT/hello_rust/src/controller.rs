use crate::repositories::load_json;
use crate::repositories::save_json;
use crate::views::view;
use crate::models::Task;
use std::error::Error;
use crate::views::add;

pub fn control_without_add() -> Result<(), Box<dyn Error>>{
    let tasks=load_json()?;
    view(&tasks);  
    Ok(())
}

pub fn control_with_add() -> Result<(), Box<dyn Error>>{
    let mut tasks=load_json()?;
    let description=add();
    let new_task = Task {
        id: (tasks.len() as u32) + 1,
        description, 
        completed: false,
    };
    tasks.push(new_task);
    save_json(&tasks)?;
    println!("Task added successfully!");
    Ok(())
}