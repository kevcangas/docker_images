fn main() {
    // --- CASO 1: Tipos Simples (Stack) ---
    // Los enteros son simples, tienen tamaño fijo.
    let x = 5;
    let y = x; 
    // Aquí Rust hace una COPIA del valor 5.
    // Tanto 'x' como 'y' son dueños de sus propios 5.
    println!("Stack: x vale {} y, y vale {}", x, y); // ¡Funciona!

    
    // --- CASO 2: Tipos Complejos (Heap) ---
    // Un String se guarda en el Heap (memoria dinámica).
    let s1 = String::from("Hola");
    
    // ¡AQUÍ OCURRE LA MAGIA DEL OWNERSHIP!
    // No se copia el texto. Se "mueve" la propiedad de s1 a s2.
    // s1 deja de ser válido inmediatamente.
    let s2 = s1; 
    
    // Si descomentas la siguiente línea, el compilador te dará error:
    // println!("s1 vale: {}", s1); // ERROR: s1 ya no es dueño de nada.
    
    println!("Heap: s2 vale: {}", s2); // s2 es el nuevo y único dueño.


    // --- CASO 3: Ownership y Funciones ---
    let s3 = String::from("Rust");
    
    // Al pasar s3 a la función, le estamos REGALANDO la propiedad.
    tomar_propiedad(s3); 
    
    // s3 ya no es válido aquí. Ya no "tienes" el libro, lo regalaste.
    // println!("{}", s3); // Esto daría error.

    
    // --- CASO 4: Devolver Ownership ---
    let s4 = String::from("Vuelve");
    // Pasamos el valor, la función lo usa y nos lo devuelve.
    let s5 = tomar_y_devolver(s4);
    println!("Me devolvieron: {}", s5);
}

fn tomar_propiedad(texto: String) {
    // 'texto' ahora es el dueño del String.
    println!("La función 'tomar_propiedad' dice: {}", texto);
} // Aquí termina la función, 'texto' sale del ámbito y Rust libera la memoria.

fn tomar_y_devolver(texto: String) -> String {
    println!("La función 'tomar_y_devolver' usó: {}", texto);
    texto // Devolvemos la propiedad a quien llamó la función
}