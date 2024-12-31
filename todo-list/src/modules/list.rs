use super::task::Task;

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
}