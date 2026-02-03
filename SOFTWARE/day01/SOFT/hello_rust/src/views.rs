use crate::models::Task;

pub fn view(tasks:&[Task]){
    for task in tasks{
        let status:bool=task.completed;
    match status{
        true => println!("-- {} -- [{}] {}",task.id,"X",task.description),
        false => println!("-- {} -- [{}] {}",task.id," ",task.description)
    }
    }
}
use std::io;
pub fn add() -> String{
    println!("Enter the task description:");
    let mut task=String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");
    task.trim().to_string()
}