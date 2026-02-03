use std::io;
use crate::controller::control_without_add;
use crate::controller::control_with_add;
use crate::controller::control_with_update;
use crate::controller::control_with_delete;

pub fn router(){
    println!("Welcome to your Task Manager!");
    loop{println!("What do you want to do?
                    1 - List all tasks
                    2 - Add a task
                    3 - Leave
                    4 - Update a task
                    5 - Delete a task");
    let mut number=String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    match number.trim()
    {
        "1" => if let Err(e)=control_without_add() {println!("Error {}",e);},
        "2" => if let Err(e)=control_with_add() {println!("Error {}",e);},
        "3" => {
                    println!("See you!");
                    break;
                },
        "4" =>if let Err(e)=control_with_update() {println!("Error {}",e);},
        "5" =>if let Err(e)=control_with_delete() {println!("Error {}",e);}
        _ => println!("Type 1, 2, or 3.")

}
}
}