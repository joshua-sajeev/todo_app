use todo::{help, Todo};
use std::{env,process};
fn main() {
    let mut todo = Todo::new().expect("Couldn't create the todo instance");

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        todo.list();
        return;
    } else {
        let command = &args[1];
        match &command[..] {
            "list"|"-l" => {
                todo.list();
            }
            "add"|"-a" => {
                todo.add(&args[2..]);
            }
            "rm"|"-r" => {
                todo.remove(&args[2..]);
            }
            "done"|"-d" => {
                todo.done(&args[2..]);
            }
            "done-list" => {
                todo.list_done();
            }
            "sort"|"-s"=> {
                todo.sort();
            }
            "clear"|"-c" => {
                todo.clear();
            }
            "restore" => {
                todo.restore();
            }
            "help" | "--help" | "-h" => {
                help();
            }
            _ => {
                eprintln!("Unknown command. Use 'help' for usage information.");
                process::exit(1);
            }
        }
    }
}

