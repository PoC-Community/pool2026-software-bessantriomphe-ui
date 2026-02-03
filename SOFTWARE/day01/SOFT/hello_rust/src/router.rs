use std::io;
use crate::controller::control_without_add;
use crate::controller::control_with_add;

pub fn router(){
    println!("Welcome to your Task Manager!");
    loop{println!("What do you want to do?
            1 - List all tasks
            2 - Add a task
            3 - Leave");
    let mut number=String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    match number.trim()
    {
        "1" => control_without_add().unwrap(),
        "2" =>control_with_add().unwrap(),
        "3" => {
                    println!("See you!");
                    break;
                },
    
        _ => println!("Type 1, 2, or 3.")

}
}
}