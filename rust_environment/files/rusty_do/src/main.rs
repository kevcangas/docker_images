use serde::{Deserialize, Serialize};
use std::fs; // Para manejo de archivos
use std::env; // Para leer argumentos de la terminal


// 1. Definición de una "Tarea"
// "Derive" le dice a Rust que implemente automaticamente la conversión a JSON
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(desc: String) -> Task {
        Task{
            description: desc,
            completed: false,
        }
    }
}

// 2. Función para guardar tareas en un archivo
fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).expect("Error in creating JSON");
    
    let write = fs::write("data/db.json",&json);

    match write {
        Ok(_) => println!("Successfully wrote"),
        Err(_) => {
            fs::create_dir("data").expect("Directory was not created");
            fs::write("data/db.json",&json).expect("The data was not saved");
        }
    };
}

// 3. Función para cargar tareas existentes
fn load_tasks() -> Vec<Task> {
    let contents = fs::read_to_string("data/db.json");

    match contents {
        Ok(file) => {
            return serde_json::from_str(&file).expect("Error parsing JSON");
        }
        Err(_) => {
            return Vec::new();
        }
    };
}

fn main() {
// Leemos los argumentos de la terminal (ej: cargo run add "Comprar leche")
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: add <tarea> | list | done <indice>");
        return;
    }

    let command = &args[1];
    let mut tasks = load_tasks(); // Cargamos tareas viejas

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Por favor escribe la descripción de la tarea.");
            } else {
                let desc = &args[2];
                let new_task = Task::new(desc.to_string());
                tasks.push(new_task);
                save_tasks(&tasks); // Guardamos cambios
                println!("Tarea agregada!");
            }
        }
        "list" => {
            // TODO: Itera sobre el vector 'tasks'.
            // Imprime el índice y si está completada [x] o no [ ].
            // Ejemplo de salida: "1. [x] Aprender Rust"
            for (num,task) in tasks.iter().enumerate() {
                println!("{}. [{}] {}",num,if task.completed {"x".to_string()} else {"".to_string()}, task.description);
            }

        }
        "done" => {
            // TODO: Parsea el argumento 2 (el índice) a número.
            // Accede al vector y cambia 'completed' a true.
            // Recuerda manejar el caso si el índice no existe (index out of bounds).
            // No olvides llamar a save_tasks(&tasks) al final.
            let index = &args[2].parse::<usize>().expect("The arg is not a number");
            if *index < tasks.len(){
                tasks[*index].completed = true;
                save_tasks(&tasks);
            }
            else {
                println!("The index doesn't exist");
            }
            

        }
        _ => println!("Comando desconocido."),
    }
}
