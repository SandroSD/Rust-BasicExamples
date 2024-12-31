mod modules;

use modules::list::List;
use modules::task::Task;

use std::io::{self, Write};
use std::process;

fn save_user_input() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string() // trim returns a reference, so when the fn finish it's execution, the reference is going to point to something that will not exist anymore, that is why we need to_string()
}

fn main() {
    let string_input = r#"Please write the operation:
    - Add new task (a/A)
    - List all tasks (l/L)
    - Mark task as done (m/M)
    - Delete task (d/D)
    - Exit (e/E)
    - Help (h/H)
    - Clear (c/C)
    - Save (s/S)
    - Load (lo/LO)
    - Search (se/SE)
    - Edit (ed/ED)
    - Sort (so/SO)
    - Filter (f/F)
    - Undo (u/U)
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
        }
        
        /*else if action == "e" {
            println!("Finish execution.");
            //process::exit(1);
        } else if action == "h" {
            println!("{}", string_input);
        } else if action == "c" {
            println!("Clear the list of tasks.");
            list.clear();
        } else if action == "s" {
            println!("Save the list of tasks.");
            list.save();
        } else if action == "lo" {
            println!("Load the list of tasks.");
            list.load();
        } else if action == "se" {
            println!("Search for a task.");
            println!("Write the name of the task");
            let name = save_user_input();
            list.search(name);
        } else if action == "ed" {
            println!("Edit a task.");
            println!("Write the number of the task");
            let number_of_the_task = save_user_input().to_lowercase();
            list.edit(number_of_the_task);
        } else if action == "so" {
            println!("Sort the list of tasks.");
            list.sort();
        } else if action == "f" {
            println!("Filter the list of tasks.");
            list.filter();
        } else if action == "u" {
            println!("Undo the last operation.");
            list.undo();
        }*/
        

        println!("{}", string_input);

        action = save_user_input().to_lowercase();
    }

}
