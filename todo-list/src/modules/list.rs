use super::task::Task;
use crate::utils::save_user_input;

#[derive(Debug)]
pub struct List {
    tasks: Vec<Task>
}

impl List {
    pub fn new() -> Self {
        List { tasks: vec![] }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            task.read();
        }
    }

    pub fn mark_task_as_done(&mut self, last_4_id_digits: String) {
        if let Some(pos) = self.tasks.iter().position(|x| x.get_id().to_string().ends_with(&last_4_id_digits)) {
            self.tasks[pos].complete();

            self.list_tasks();
        }
    }

    pub fn delete_task(&mut self, last_4_id_digits: String) {
        if let Some(pos) = self.tasks.iter().position(|x| x.get_id().to_string().ends_with(&last_4_id_digits)) {
            self.tasks.remove(pos);

            self.list_tasks();
        }
    }

    pub fn search(&self, name: String) {
        for task in &self.tasks {
            if task.get_name() == name || task.get_name().contains(&name) {
                task.read();
            }
        }
    }

    pub fn edit(&mut self, last_4_id_digits: String) {
        if let Some(pos) = self.tasks.iter().position(|x| x.get_id().to_string().ends_with(&last_4_id_digits)) {
            println!("Write the new name of the task");
            let name = save_user_input();
    
            println!("Write the new description of the task");
            let description = save_user_input();
    
            println!("Write the new priority of the task (1-5)");
            let priority: u8 = save_user_input().parse().expect("Error");
    
            let task = Task::new(name, description, priority);
            self.tasks[pos] = task;
        }
    }

    pub fn sort(&mut self) {
        self.tasks.sort_by(|a, b| a.get_priority().cmp(&&b.get_priority()));

        self.list_tasks();
    }
}