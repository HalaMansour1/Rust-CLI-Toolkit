use std::env;

struct TodoItem {
    name: String,
    completed: char,
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn edit_task(&mut self, index: usize, new_name: String) {
        if let Some(item) = self.list.get_mut(index) {
            item.name = new_name;
        } else {
            println!("Invalid task index: {}", index);
        }
    }

    fn delete_task(&mut self, index: usize) {
        if index < self.list.len() {
            self.list.remove(index);
        } else {
            println!("Invalid task index: {}", index);
        }
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name);
        }
    }

    fn mark_done(&mut self, index: usize) {
        if let Some(item) = self.list.get_mut(index) {
            item.completed = if item.completed == ' ' { 'X' } else { ' ' };
        } else {
            println!("Invalid task index: {}", index);
        }
    }
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        TodoItem {
            name,
            completed: ' ',
        }
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
    Edit(usize, String),
    Delete(usize),
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    
    if arguments.len() < 2 {
        panic!("You must provide a command");
    }

    let mut todo_list = TodoList::new();

    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => {
            if arguments.len() < 3 {
                panic!("You must provide a task name");
            }
            Command::Add(arguments[2].clone())
        },
        "done" => {
            if arguments.len() < 3 {
                panic!("You must provide a task index");
            }
            Command::Done(arguments[2].parse().expect("Invalid index"))
        },
        "edit" => {
            if arguments.len() < 4 {
                panic!("You must provide a task index and new name");
            }
            Command::Edit(
                arguments[2].parse().expect("Invalid index"),
                arguments[3].clone(),
            )
        },
        "delete" => {
            if arguments.len() < 3 {
                panic!("You must provide a task index");
            }
            Command::Delete(arguments[2].parse().expect("Invalid index"))
        },
        _ => panic!("Invalid command"),
    };

    // Sample tasks for demonstration
    todo_list.add_to_list("Clean macbook".to_string());
    todo_list.add_to_list("Build crazy rust stuff".to_string());

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
        }
        Command::Edit(index, new_name) => {
            todo_list.edit_task(index, new_name);
            todo_list.print();
        }
        Command::Delete(index) => {
            todo_list.delete_task(index);
            todo_list.print();
        }
    }
}
