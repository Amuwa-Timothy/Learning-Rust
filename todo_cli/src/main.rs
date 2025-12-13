fn display_task (tasks: &Vec<String>) {

    if tasks.is_empty() {
        println!("No tasks yet!");
    }else{
        for (index, item) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, item);           
        }
    }

}



fn main() {
    // Test with empty list
    let empty_tasks: Vec<String> = Vec::new();
    display_task(&empty_tasks);
    
    println!(); // blank line
    
    // Test with some tasks
    let mut tasks = Vec::new();
    tasks.push(String::from("Buy milk"));
    tasks.push(String::from("Walk dog"));
    tasks.push(String::from("Code Rust"));
    display_task(&tasks);
}
