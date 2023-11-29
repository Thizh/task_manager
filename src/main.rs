use std::io;

struct Task {
    title: String,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    println!("Task Manager");

    loop {
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Remove Task");
        println!("5. Quit");

        let choice: u32 = get_user_input("Enter your choice: ").parse().expect("Invalid input. Please enter a number.");

        match choice {
            1 => {
                let title = get_user_input("Enter task title:");
                let description = get_user_input("Enter task description:");
                add_task(&mut tasks, &title, &description);
            }
            2 => list_tasks(&tasks),
            // Add cases for marking tasks as completed, removing tasks, and quitting
            // ...
            5 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn add_task(tasks: &mut Vec<Task>, title: &str, description: &str) {
    let new_task = Task {
        title: title.to_string(),
        description: description.to_string(),
        completed: false,
    };
    tasks.push(new_task);
}

fn list_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. [{}] {}: {}", index + 1, if task.completed { "X" } else { " " }, task.title, task.description);
    }
}

