pub mod utils;
pub mod todo;
use sqlite::Connection;

fn main() {
    // println!("RUSTY DO");
    utils::display_title();
    println!("By NoobScience 2023");
    // println!("\n");
    let connection = Connection::open("todo.db").unwrap();
    let db = utils::Db{connection: &connection};
    db.db_init();
    loop {
        let mut choice = String::new();
        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        println!("add - Add a todo");
        println!("show - Display all todos");
        println!("mark - Mark a todo as completed");
        println!("del - Delete a todo");
        println!("exit - Exit");
        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice= choice.trim();
        match choice {
            "add" => {
                let todo = todo::Todo::ask_input();
                db.add_todo(&todo);
            },
            "show" => {
                db.display_all();
            },
            "mark" => {
                println!("Enter the id of the todo to mark as completed:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                let id: i32 = id.trim().parse().unwrap();
                db.mark_complete(id);
            },
            "del" => {
                println!("Enter the id of the todo to delete:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                let id: i32 = id.trim().parse().unwrap();
                db.delete(id);
            },
            "exit" => {
                println!("Thank you for using Rusty Do!");
                break;
            },
            _ => {
                println!("Invalid choice!");
            }
        }
    }
}