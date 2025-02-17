use std::io;

struct Task {
    title: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\nSimple To-Do App");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter task name:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input");

    let task = Task {
        title: title.trim().to_string(),
        completed: false,
    };
    tasks.push(task);
    println!("Task added!");
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        println!("Your Tasks:");
        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {} [{}]", index + 1, task.title, if task.completed { "✔" } else { "✘" });
        }
    }
}
