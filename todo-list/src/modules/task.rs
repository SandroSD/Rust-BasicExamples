use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    id: Uuid,
    name: String,
    description: String,
    is_completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    priority: u8
}

impl Task {
    pub fn new(name: String, description: String, priority: u8) -> Self {
        Task {
            id: Uuid::new_v4(),
            name,
            description,
            is_completed: false,
            created_at: Utc::now(),
            completed_at: None,
            priority
        }
    }

    pub fn read(&self) {
        println!("Id: {}", self.id);
        println!("Task: {}", self.name);
        println!("Description: {}", self.description);
        println!("Priority: {}", self.priority);
        println!("Created at: {}", self.created_at);
        println!("Completed at: {:?} \n\n", self.completed_at);
    }

    pub fn complete(&mut self) {
        self.is_completed = true;
        self.completed_at = Some(Utc::now());
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_priority(&self) -> u8 {
        self.priority
    }
}