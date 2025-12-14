use std::io;
use std::fs;

//create the function responsible for displaying the current tasks
fn display_task (tasks: &Vec<String>) {

    if tasks.is_empty() {
        println!("No tasks yet!");
    }else{
        for (index, item) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, item);           
        }
    }

}

//create the function responsible for adding new tasks
fn add_task (tasks: &mut Vec<String>) {

    println!( "Enter a new task:");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    tasks.push(input.trim().to_string());
    println!("Task created!")
}


//create the function responsible for removing tasks
fn remove_task (tasks: &mut Vec<String>) {

    display_task(&tasks);
    println!("Which number do you want to remove");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let user_choice: usize = input.trim().parse().unwrap_or(0);

    if user_choice  >= 1 && user_choice <= tasks.len() {
        tasks.remove(user_choice - 1);
        println!("Task Removed!");
    }else{
        println!("Invalid Choice!");
        return;
    }  

}


//create a function that stores all tasks created
fn save_task(tasks: &Vec<String>) {

    let task = tasks.join("\n");
    fs::write("tasks.txt", task).expect("Failed to write file");
}


//A function that looks for a file that has previous todo tasks
//if there is none it creates one
fn load_task() -> Vec<String> {
    match fs::read_to_string("tasks.txt") {
        Ok(contents) => {
            let task: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
            task
        },
        Err(_) => {
            Vec::new()
        }
    }
      
}


fn main() {

    let mut tasks: Vec<String> = load_task(); //Loading the tasks from a file if it exist

    loop {
        display_task(&tasks);
        println!("What do you want to do? Enter your choice using the number option: ");
        println!("1. Add task, 2. Remove task, 3. Exit");

        let mut input: String = String::new();//Getting user input and converting it to int
        io::stdin().read_line(&mut input).expect("Failed to read");
        let user_choice: usize = input.trim().parse().unwrap_or(0);

        match user_choice {
            1 => {
                add_task(&mut tasks);
                save_task(&tasks);
            },
            2 => {
                remove_task(&mut tasks);
                save_task(&tasks);
            },
            3 => {
                break;
            },
            _ => {
                println!("Invalid Choice");
            }

        }
    }
}



