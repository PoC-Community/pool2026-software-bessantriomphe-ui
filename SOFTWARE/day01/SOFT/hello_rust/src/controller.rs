use crate::repositories::load_json;
use crate::repositories::save_json;
use crate::views::update;
use crate::views::view;
use crate::views::delete;
use crate::models::Task;
use std::error::Error;
use crate::views::add_desc;
use crate::views::add_prio;
use crate::views::add_tag;
use std::io;

pub fn control_without_add() -> Result<(), Box<dyn Error>>{
    let tasks=load_json()?;
    view(&tasks);  
    Ok(())
}

pub fn control_with_add() -> Result<(), Box<dyn Error>>{
    let mut tasks=load_json()?;
    let description=add_desc();
    let priority=add_prio();
    let tags= add_tag();
    let new_task = Task {
        id: (tasks.len() as u32) + 1,
        description, 
        completed: false,
        created_at: chrono::Utc::now(),
        priority,
        tags
    };
    tasks.push(new_task);
    save_json(&tasks)?;
    println!("Task added successfully!");
    Ok(())
}


pub fn control_with_update() ->Result<(), Box<dyn Error>>{
    let mut tasks=load_json()?;
    let id_recherche=update().parse::<u32>()
        .expect("Unable to parse the id");

    let task=tasks.iter_mut().find(|t| t.id == id_recherche);
    if let Some(task)=task{
        println!("1-Mark as done\n2-Edit description\n3-Update priority\n4-Update tag");
        let mut number=String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        match number.trim(){
            "1" => task.completed=true,
            "2" => {
                   let newdescription= add_desc();
                    task.description=newdescription;
                    },
            "3" => task.priority=add_prio(),
            "4" => task.tags=add_tag(),
            _ => println!("Invalid choice"),
            }
        }
    save_json(&tasks)?;
    Ok(())
}

pub fn control_with_delete() ->Result<(), Box<dyn Error>>{
        let mut tasks=load_json()?;
    let id_recherche=delete().parse::<u32>()
        .expect("Unable to parse the id");

    tasks.retain(|t| t.id != id_recherche);
    save_json(&tasks)?;
    Ok(())
}