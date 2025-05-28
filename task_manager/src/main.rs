use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Local};

const MAX_TASKS: usize = 100;
const MAX_NAME_LENGTH: usize = 100;
const TASKS_FILE: &str = "tasks.json";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    name: String,
    priority: Priority,
    completed: bool,
    due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn load_tasks() -> io::Result<Self> {
        if Path::new(TASKS_FILE).exists() {
            let content = fs::read_to_string(TASKS_FILE)?;
            Ok(serde_json::from_str(&content).unwrap_or_else(|_| Self::new()))
        } else {
            Ok(Self::new())
        }
    }

    fn save_tasks(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(TASKS_FILE, json)?;
        Ok(())
    }

    fn add_task(&mut self, name: String, priority: Priority, due_date: Option<String>) -> io::Result<()> {
        if self.tasks.len() >= MAX_TASKS {
            println!("Cannot add more tasks. Maximum limit ({}) reached.", MAX_TASKS);
            return Ok(());
        }

        if name.len() > MAX_NAME_LENGTH {
            println!("Task name too long. Maximum length is {} characters.", MAX_NAME_LENGTH);
            return Ok(());
        }

        let task = Task {
            name,
            priority,
            completed: false,
            due_date,
        };

        self.tasks.push(task);
        self.save_tasks()?;
        println!("Task added successfully!");
        Ok(())
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        println!("\nCurrent Tasks:");
        println!("-------------");
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "✓" } else { " " };
            let priority = match task.priority {
                Priority::Low => "Low",
                Priority::Medium => "Medium",
                Priority::High => "High",
            };
            let due_date = task.due_date.as_deref().unwrap_or("No due date");
            println!(
                "{}. [{}] {} (Priority: {}, Due: {}) {}",
                index + 1,
                status,
                task.name,
                priority,
                due_date,
                if task.completed { "(Completed)" } else { "" }
            );
        }
        println!("-------------\n");
    }

    fn complete_task(&mut self, index: usize) -> io::Result<()> {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
            self.save_tasks()?;
            println!("Task marked as completed!");
        } else {
            println!("Task not found!");
        }
        Ok(())
    }

    fn delete_task(&mut self, index: usize) -> io::Result<()> {
        if index < self.tasks.len() {
            print!("Are you sure you want to delete this task? (y/n): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if input.trim().to_lowercase() == "y" {
                self.tasks.remove(index);
                self.save_tasks()?;
                println!("Task deleted successfully!");
            } else {
                println!("Deletion cancelled.");
            }
        } else {
            println!("Task not found!");
        }
        Ok(())
    }
}

fn get_priority() -> Priority {
    loop {
        println!("Enter priority:");
        println!("1. Low");
        println!("2. Medium");
        println!("3. High");
        print!("Choose (1-3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return Priority::Low,
            "2" => return Priority::Medium,
            "3" => return Priority::High,
            _ => println!("Invalid input. Please enter 1, 2, or 3."),
        }
    }
}

fn get_due_date() -> Option<String> {
    loop {
        println!("Would you like to add a due date? (y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "n" => return None,
            "y" => {
                println!("Enter due date (YYYY-MM-DD), or press Enter to skip: ");
                let mut date_input = String::new();
                io::stdin().read_line(&mut date_input).unwrap();
                let date_input = date_input.trim();

                if date_input.is_empty() {
                    return None;
                }

                match NaiveDate::parse_from_str(date_input, "%Y-%m-%d") {
                    Ok(date) => {
                        if date >= Local::now().naive_local().date() {
                            return Some(date_input.to_string());
                        } else {
                            println!("Due date cannot be in the past!");
                        }
                    }
                    Err(_) => println!("Invalid date format. Please use YYYY-MM-DD."),
                }
            }
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}

fn main() -> io::Result<()> {
    let mut task_manager = TaskManager::load_tasks()?;

    loop {
        println!("\nTask Manager Menu:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Exit");
        print!("\nChoose an option (1-5): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                print!("Enter task name: ");
                io::stdout().flush()?;
                let mut name = String::new();
                io::stdin().read_line(&mut name)?;
                let name = name.trim().to_string();

                if name.is_empty() {
                    println!("Task name cannot be empty!");
                    continue;
                }

                let priority = get_priority();
                let due_date = get_due_date();
                task_manager.add_task(name, priority, due_date)?;
            }
            "2" => task_manager.list_tasks(),
            "3" => {
                task_manager.list_tasks();
                print!("Enter task number to mark as completed: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if let Ok(index) = input.trim().parse::<usize>() {
                    task_manager.complete_task(index - 1)?;
                } else {
                    println!("Invalid task number!");
                }
            }
            "4" => {
                task_manager.list_tasks();
                print!("Enter task number to delete: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if let Ok(index) = input.trim().parse::<usize>() {
                    task_manager.delete_task(index - 1)?;
                } else {
                    println!("Invalid task number!");
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please choose 1-5."),
        }
    }

    Ok(())
}
