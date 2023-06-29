pub struct Todo{
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todo{
    pub fn new(id: i32, title: String, completed: bool) -> Todo{
        return Todo{
            id,
            title,
            completed
        }
    }

    pub fn new_empty() -> Todo{
        return Todo{
            id: 0,
            title: String::new(),
            completed: false
        }
    }

    pub fn print(&self){
        println!("{} - {} - {}", self.id, self.title, self.completed);
    }

    pub fn ask_input()->Todo{
        println!("Enter a new todo item:");
        let mut title = String::new();
        std::io::stdin().read_line(&mut title).unwrap();
        let title = title.trim();
        return Todo{
            id: 0,
            title: title.to_string(),
            completed: false
        }
    }
}