use sqlite::Connection;

use crate::todo;

//? We'll be using a life time parameter to ensure that the connection
// is not dropped before the Db struct.
// This is a common pattern in Rust, when we pass
// a reference to a struct to another struct.
pub struct Db<'a>{
    pub connection: &'a Connection
}

impl Db<'_> {    
    pub fn db_init(&self){
        self.connection.execute(
            "
            CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL
            );
            "
        ).unwrap();
    }

    pub fn db_test(&self){
        self.connection.execute(
            "
            INSERT INTO todo (title, completed) VALUES ('test', 0);
            "
        ).unwrap();
    }

    pub fn add_todo(&self, todo: &todo::Todo){
        self.connection.execute(
            format!(
                "
                INSERT INTO todo (title, completed) VALUES ('{}', {});
                ",
                todo.title,
                todo.completed
            ).as_str()
        ).unwrap();
    }

    pub fn display_all(&self){
        println!("=============================================================");
        println!("\t{}  \t\t{}  \t\t\t{}", "id", "title", "Status");
        println!("-------------------------------------------------------------");
        self.connection.iterate(
            "SELECT * from todo;", |pairs| {
                for &(_column, value) in pairs.iter() {
                    if _column == "completed"{
                        if value.unwrap() == "0"{
                            print!("\t{}\t", "Pending");
                        }else{
                            print!("\t{}\t", "Completed");
                        }
                        continue;
                    }
                    else{
                        print!("\t{}\t",value.unwrap());
                    }
                }
                print!("{}\n", " ");
                true
            }
        ).unwrap();
        println!("=============================================================");
    }

    pub fn delete(&self, id: i32){
        self.connection.execute(
            format!(
                "
                DELETE FROM todo WHERE id = {};
                ",
                id
            ).as_str()
        ).unwrap();
    }

    pub fn mark_complete(&self, id: i32){
        self.connection.execute(
            format!(
                "
                UPDATE todo SET completed = 1 WHERE id = {};
                ",
                id
            ).as_str()
        ).unwrap();
    }
}

pub fn display_title(){
    print!("{}", "
 ____  _   _ ____ _______   __  ____   ___  
|  _ \\| | | / ___|_   _\\ \\ / / |  _ \\ / _ \\ 
| |_) | | | \\___ \\ | |  \\ V /  | | | | | | |
|  _ <| |_| |___) || |   | |   | |_| | |_| |
|_| \\_\\\\___/|____/ |_|   |_|   |____/ \\___/
   
    ")
}