# Task Manager

A simple command-line task manager written in Rust that helps you keep track of your daily tasks.

## Features

- Add tasks with name, priority, and optional due date
- List all tasks with their status and details
- Mark tasks as completed
- Delete tasks
- Persistent storage using JSON
- User-friendly interface
- Input validation and error handling

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone this repository or download the source code
2. Navigate to the project directory:
   ```bash
   cd task_manager
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Running the Program

To run the program, use:

```bash
cargo run
```

## Usage

The program presents a menu with the following options:

1. Add Task - Create a new task with a name, priority level, and optional due date
2. List Tasks - Display all tasks with their details
3. Complete Task - Mark a task as completed
4. Delete Task - Remove a task from the list
5. Exit - Close the program

### Task Properties

- Name: Limited to 100 characters
- Priority: Low, Medium, or High
- Due Date: Optional, format YYYY-MM-DD
- Status: Pending or Completed

### Data Storage

Tasks are automatically saved to a `tasks.json` file in the same directory as the program. This file is loaded when the program starts and updated after each modification.

## Example Usage

```
Task Manager Menu:
1. Add Task
2. List Tasks
3. Complete Task
4. Delete Task
5. Exit

Choose an option (1-5): 1
Enter task name: Buy groceries
Enter priority:
1. Low
2. Medium
3. High
Choose (1-3): 2
Would you like to add a due date? (y/n): y
Enter due date (YYYY-MM-DD), or press Enter to skip: 2024-03-20
Task added successfully! 