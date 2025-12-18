use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::sync::mpsc; // "Multi-Producer, Single-Consumer" (Canales)
use std::io::{self, Write};
use colored::*;

// Configuración
const MAX_PORT: u16 = 1000; // Escanearemos del puerto 1 al 1000

fn main() {
    
    // 1. Leer argumentos (IP a escanear)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: cargo run <ip_address>");
        return;
    }
    let ip = &args[1];

    println!("Escaneando {} ...", ip);

    // 2. Crear un CANAL (Channel) para comunicación
    // tx = Transmisor (lo que usan los hilos para gritar "¡Encontré uno!")
    // rx = Receptor (lo que usa el hilo principal para escuchar y anotar)
    let (tx, rx) = mpsc::channel();

    // 3. Lanzar hilos
    for port in 1..=MAX_PORT {
        // Clonamos el transmisor y la IP para que cada hilo tenga su propia copia
        let tx = tx.clone();
        let ip = ip.to_string();

        // spawn crea un nuevo hilo
        thread::spawn(move || {
            let stream = TcpStream::connect(format!("{}:{}", ip, port));
            match stream {
                Ok(_) => { tx.send(port).unwrap(); }
                Err(_) => {}
            }
        });
    }

    // Cerramos el transmisor original para que el receptor sepa cuándo dejar de esperar
    drop(tx);

    // 4. Recolectar resultados
    let mut open_ports = Vec::new();
    println!("Recopilando reusltados...");

    // Este bucle espera mensajes del canal hasta que todos los transmisores se cierren
    for port in rx {
        open_ports.push(port);
        // Imprimimos un punto . para ver progreso sin llenar la pantalla
        print!("."); 
        io::stdout().flush().unwrap();
    }

    // 5. Mostrar reporte final
    println!("\n\nResultados para {}:", ip.green());
    open_ports.sort();
    for port in open_ports {
        println!("Puerto {} está {}", port, "ABIERTO".green().bold());
    }
}
