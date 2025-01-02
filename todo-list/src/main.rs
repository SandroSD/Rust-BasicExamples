mod modules;
mod utils;

use modules::list::List;
use modules::task::Task;
use utils::save_user_input;

use std::process;

fn main() {
    let string_input = r#"Please write the operation:
    - Add new task (a/A)
    - List all tasks (l/L)
    - Mark task as done (m/M)
    - Delete task (d/D)
    - Exit (e/E)
    - Search (se/SE)
    - Edit (ed/ED)
    - Sort (so/SO)
    "#;

    println!("{}", string_input);
    
    let mut action = save_user_input().to_lowercase();

    let mut list:List = List::new();

    while action != "e" {
        if action == "a" {
            println!("Write the name of the task");
            let name = save_user_input();
    
            println!("Write the description of the task");
            let description = save_user_input();
    
            println!("Write the priority of the task (1-5)");
            let priority: u8 = save_user_input().parse().expect("Error");
    
            let task = Task::new(name, description, priority);
            list.add_task(task);
        } else if action == "l" {
            list.list_tasks();
        } else if action == "m" {
            println!("Write the last 4 digits of the task's ID.");
            let last_4_id_digits = save_user_input().to_lowercase();
            list.mark_task_as_done(last_4_id_digits);
        } else if action == "d" {
            println!("Write the last 4 digits of the task's ID.");
            let last_4_id_digits = save_user_input().to_lowercase();
            list.delete_task(last_4_id_digits);
        } else if action == "e" {
            process::exit(1);
        } else if action == "se" {
            println!("Write the name of the task");
            let name = save_user_input();
            list.search(name);
        } else if action == "ed" {
            println!("Write the last 4 digits of the task's ID.");
            let last_4_id_digits = save_user_input().to_lowercase();
            list.edit(last_4_id_digits);
        } else if action == "so" {
            println!("Sort the list of tasks.");
            list.sort();
        }  

        println!("{}", string_input);

        action = save_user_input().to_lowercase();
    }

}
