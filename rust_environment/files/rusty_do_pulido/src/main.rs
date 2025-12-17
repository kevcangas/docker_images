use serde::{Deserialize, Serialize};
use std::fs;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(desc: String) -> Task {
        Task { description: desc, completed: false }
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).expect("Error in creating JSON");
    
    // NUEVO: fs::create_dir_all es más robusto. 
    // Crea toda la ruta si falta y NO da error si ya existe.
    // Así te ahorras el if/else o el match complejo.
    fs::create_dir_all("data").ok(); 
    
    fs::write("data/db.json", json).expect("Failed to write to file");
}

fn load_tasks() -> Vec<Task> {
    let contents = fs::read_to_string("data/db.json");
    match contents {
        Ok(file) => serde_json::from_str(&file).expect("Error parsing JSON"),
        Err(_) => Vec::new(),
    } // Nota: quité los 'return' explícitos, es más "Rustacean"
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: add <tarea> | list | done <indice>");
        return;
    }

    let command = &args[1];
    let mut tasks = load_tasks();

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Por favor escribe la descripción.");
            } else {
                let desc = &args[2];
                tasks.push(Task::new(desc.to_string()));
                save_tasks(&tasks);
                println!("Tarea agregada!");
            }
        }
        "list" => {
            // NUEVO: Usamos .iter() para no destruir el vector
            for (id, task) in tasks.iter().enumerate() {
                // NUEVO: Lógica visual más limpia sin crear Strings nuevos
                let status = if task.completed { "[x]" } else { "[ ]" };
                println!("{}. {} {}", id, status, task.description);
            }
        }
        "done" => {
            if args.len() < 3 {
                println!("Indica el número de la tarea.");
                return;
            }
            // NUEVO: Manejo seguro si el usuario no pone un número
            match args[2].parse::<usize>() {
                Ok(index) => {
                    if index < tasks.len() {
                        tasks[index].completed = true;
                        save_tasks(&tasks);
                        println!("Tarea {} completada.", index);
                    } else {
                        println!("La tarea {} no existe.", index);
                    }
                },
                Err(_) => println!("Error: '{}' no es un número válido.", args[2]),
            }
        }
        _ => println!("Comando desconocido."),
    }
}