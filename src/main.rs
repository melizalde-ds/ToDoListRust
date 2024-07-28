fn main() {
    let mut running = true;
    let mut tasks = vec![];
    while running {
        main_menu();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => tasks = add_task(tasks),
            "2" => tasks = view_task(tasks),
            "3" => tasks = remove_task(tasks),
            "4" => running = exit(),
            _ => println!("Invalid Input\n"),
        }
    }
}

fn main_menu() {
    print!("ToDo List\n");
    print!("1. Add Task\n");
    print!("2. View Task\n");
    print!("3. Remove Task\n");
    print!("4. Exit\n");
    print!("Enter your choice: \n");
}

fn add_task(mut tasks: Vec<String>) -> Vec<String> {
    print!("Add Task\n");
    print!("Enter task: \n");
    let mut task = String::new();
    std::io::stdin().read_line(&mut task).unwrap();
    let task = task.trim();
    tasks.push(task.to_string());
    println!("Task added successfully\n");
    tasks
}

fn view_task(tasks: Vec<String>) -> Vec<String> {
    println!("View Tasks:\n");
    if tasks.is_empty() {
        println!("No tasks available\n");
        return tasks;
    }
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
    tasks
}

fn remove_task(tasks: Vec<String>) -> Vec<String> {
    println!("Remove Task\n");
    print!("Enter task number to remove: \n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let index = input.parse::<usize>().unwrap();
    if index > tasks.len() || index < 1 {
        println!("Invalid task number\n");
        return tasks;
    }
    let task = tasks.get(index - 1).unwrap();
    println!("Task removed: {}\n", task);
    let tasks = tasks
        .into_iter()
        .enumerate()
        .filter_map(|(i, t)| if i != index - 1 { Some(t) } else { None })
        .collect();
    tasks
}

fn exit() -> bool {
    println!("Thank you!\n");
    false
}
