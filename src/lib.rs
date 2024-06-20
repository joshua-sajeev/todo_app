use std::{fs, fmt};
use fancy::printcoln;
use std::{process, env};
use std::fs::OpenOptions;
use std::io::{BufWriter, BufReader, Write, Read};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Status {
    Active,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "Active"),
            Status::Done => write!(f, "Done"),
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    task: String,
    status: Status,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.task, self.status)
    }
}

#[derive(Debug)]
pub struct Todo {
    pub todo: Vec<Entry>,
    pub todo_path: String,
    pub todo_bak: String,
    pub no_backup: bool,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                let home = env::var("HOME").map_err(|_| "HOME environment variable not set")?;

                let legacy_todo = format!("{}/TODO", &home);
                if Path::new(&legacy_todo).exists() {
                    legacy_todo
                } else {
                    format!("{}/.todo", &home)
                }
            }
        };

        let todo_bak: String = match env::var("TODO_BAK_DIR") {
            Ok(t) => t,
            Err(_) => String::from("/tmp/todo.bak"),
        };

        let no_backup = env::var("TODO_NOBACKUP").is_ok();

        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .map_err(|_| "Couldn't open the todofile")?;

        let mut buf_reader = BufReader::new(&todofile);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).map_err(|_| "Failed to read todofile")?;

        let todo = contents
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let task = parts[0].trim().to_string();
                    let status = match parts[1].trim().to_lowercase().as_str() {
                        "done" => Status::Done,
                        _ => Status::Active,
                    };
                    Entry { task, status }
                } else {
                    Entry {
                        task: line.to_string(),
                        status: Status::Active,
                    }
                }
            })
            .collect();

        Ok(Self {
            todo,
            todo_path,
            todo_bak,
            no_backup,
        })
    }

    pub fn list(&self) {
        if self.todo.is_empty() {
            println!("No tasks in the TODO list.");
        } else {
            for (index, entry) in self.todo.iter().enumerate() {
                match entry.status {
                    Status::Active => {
                        printcoln!("[green]{}: {}", index + 1, entry.task);
                    },
                    Status::Done => {
                        printcoln!("[strikethrough][red]{}: {}", index + 1, entry.task);
                    },
                }
            }
        }
    }

    pub fn add(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);
        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            let line = format!("{}:Active\n", arg);
            buffer.write_all(line.as_bytes()).expect("unable to write data");
        }

        // Flush buffer to ensure changes are written immediately
        buffer.flush().expect("Failed to flush buffer");

        // Reload todo after modification
        *self = Todo::new().expect("Failed to reload todo list after add operation");
        self.sort();
    }

    pub fn remove(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo rm takes at least 1 argument");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);
        for (pos, entry) in self.todo.iter().enumerate() {
            let status_str = &entry.status;
            if args.contains(&(pos + 1).to_string()) {
                continue;
            }

            let line = format!("{}:{}\n", entry.task, status_str);
            buffer.write_all(line.as_bytes()).expect("unable to write data");
        }

        // Flush buffer to ensure changes are written immediately
        buffer.flush().expect("Failed to flush buffer");

        // Reload todo after modification
        *self = Todo::new().expect("Failed to reload todo list after remove operation");
        self.sort();
    }

    pub fn done(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);
        for (pos, entry) in self.todo.iter().enumerate() {
            let mut status_str = &entry.status;
            if args.contains(&(pos + 1).to_string()) {
                status_str = &Status::Done;
            }

            let line = format!("{}:{}\n", entry.task, status_str);
            buffer.write_all(line.as_bytes()).expect("unable to write data");
        }

        // Flush buffer to ensure changes are written immediately
        buffer.flush().expect("Failed to flush buffer");

        // Reload todo after modification
        *self = Todo::new().expect("Failed to reload todo list after done operation");
        self.sort();
    }

    fn remove_file(&self) {
        match fs::remove_file(&self.todo_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while clearing todo file: {}", e)
            }
        };
    }

    pub fn list_done(&self) {
        for entry in self.todo.iter() {
            if entry.status == Status::Done {
                printcoln!("[red]{}", entry.task);
            }
        }
    }

    pub fn sort(&mut self) {
        let mut done_tasks: Vec<String> = Vec::new();
        let mut active_tasks: Vec<String> = Vec::new();

        for entry in self.todo.iter() {
            let line = format!("{}:{}\n", entry.task, entry.status);
            if entry.status == Status::Done {
                done_tasks.push(line);
            } else {
                active_tasks.push(line);
            }
        }

        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);
        for task in active_tasks {
            buffer.write_all(task.as_bytes()).expect("Unable to write data");
        }
        for task in done_tasks {
            buffer.write_all(task.as_bytes()).expect("Unable to write data");
        }

        // Flush buffer to ensure changes are written immediately
        buffer.flush().expect("Failed to flush buffer");

        // Reload todo after modification
        *self = Todo::new().expect("Failed to reload todo list after sort operation");
        self.list();
    }

    fn backup(&self){
        if !self.no_backup {
            match fs::copy(&self.todo_path, &self.todo_bak) {
                Ok(_) => {},
                Err(_) => {
                    eprint!("Couldn't backup the todo file")
                }
            }
        }
    }
    pub fn clear(&mut self) {
        self.backup();
        self.remove_file();

        // Reload todo after modification
        *self = Todo::new().expect("Failed to reload todo list after clear operation");
        self.list();
    }

    pub fn restore(&mut self) {
        fs::copy(&self.todo_bak, &self.todo_path).expect("unable to restore the backup");

        // Reload todo after modification
        *self = Todo::new().expect("Failed to reload todo list after restore operation");
        self.list();
    }
}
const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Example: todo list
Available commands:
    - list| -l
        lists all tasks
        Example: todo list
    - add [TASK/s] | -a [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\" grocery
    - rm [INDEX] | -r [INDEX]
        removes a task
        Example: todo rm 4
    - done [INDEX] | -d [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - done-list
        lists all tasks that are marked done
    - sort | -s 
        sorts completed and uncompleted tasks
        Example: todo sort
    - clear | -c
        deletes all tasks
    - restore 
        restore recent backup after clear
    - help | --help | -h 
        this message
";
pub fn help() {
    // For readability
    println!("{}", TODO_HELP);
}
