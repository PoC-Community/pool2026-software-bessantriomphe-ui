use crate::models::Task;
use crate::models::Priority;
pub fn view(tasks:&[Task]){
    for task in tasks{
        let status_icon=if task.completed{"X"} else {" "};
        let tag=task.tags.join(" ");
        println!("-- {} -- [{}] {} | Priority: {:?} | Tags: [{}] | Date: {}",task.id,status_icon,task.description,task.priority,tag,task.created_at.format("%Y-%m-%d %H:%M"));

    }
    }
use std::io;
pub fn add_desc() -> String{
    println!("Enter the task description:");
    let mut task=String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");
    task.trim().to_string()
}

pub fn add_tag() -> Vec<String>{
    println!("Give a tag to your task:");
    let mut task=String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");
    task.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty()) 
        .collect()
}

pub fn add_prio() ->Priority{
    println!("What's the priority of your task?\n1-Low\n2-Medium\n3-High");
    let mut tasko=String::new();
    io::stdin()
        .read_line(&mut tasko)
        .expect("Failed to read line");
    match tasko.trim(){
        "1" => Priority::Low,
        "2" =>Priority::Medium,
        "3" => Priority::High,
        _ => Priority::Low
    }
}
pub fn update() -> String{
    println!("Enter the number of the task you want to change");
    let mut id_recherche=String::new();
    io::stdin()
        .read_line(&mut id_recherche)
        .expect("Failed to read line");
    id_recherche.trim().to_string()
}

pub fn delete() -> String{
    println!("Enter the Id of the task you want to delete");
    let mut id_recherche=String::new();
    io::stdin()
        .read_line(&mut id_recherche)
        .expect("Failed to read line");
    id_recherche.trim().to_string()
}