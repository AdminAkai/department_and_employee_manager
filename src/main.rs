use std::io;
use std::collections::HashMap;

fn main() {
    let mut run = true;
    let mut department_data: HashMap<String, Vec<String>> = HashMap::new();
    
    while run {    
        let mut department: String = String::new();
        let mut employee: String = String::new();
        let mut end_input: String = String::new();

        println!("Which department are you adding an employee to?");
        io::stdin().read_line(&mut department).expect("Failed to take input.");
        department = String::from(department.trim());

        println!("The {} department. What is the employee's name?", &department);
        io::stdin().read_line(&mut employee).expect("Failed to take input.");
        employee = String::from(employee.trim());

        department_data.entry(department).or_insert_with(|| Vec::new()).push(employee);
        
        println!("Current departments:");
        for (key, val) in &department_data {
            println!("{key}:");
            println!("{}", val.join(", "));
        }

        println!("Press any key to continue or \"quit\" to quit.");
        io::stdin().read_line(&mut end_input).expect("Failed to take input.");
        end_input = String::from(end_input.trim());

        if end_input == "quit" {
            run = false;
        }
    }
}


