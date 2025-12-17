fn main() {
    // --- ESCALARES ---
    
    // El compilador infiere que esto es i32
    let x = 5; 
    
    // Aquí especificamos explícitamente el tipo y usamos guión bajo para legibilidad
    let numero_grande: u32 = 1_000_000;
    
    // Flotante (f64 por defecto)
    let pi = 3.1416;
    
    // Booleano
    let es_rust_genial: bool = true;
    
    // Carácter (Nota las comillas simples y el emoji)
    let letra = 'z';
    let emoji = '🦀';

    // --- COMPUESTOS ---
    
    // Tupla: Mezcla un entero, un flotante y un bool
    let mi_tupla = (500, 6.5, true);
    // Destructuración: saca los valores a variables individuales
    let (a, b, c) = mi_tupla;
    
    // Array: 5 elementos, todos enteros
    let mi_array = [1, 2, 3, 4, 5];

    // --- IMPRIMIR ---
    println!("Entero x: {}", x);
    println!("Emoji: {}", emoji);
    println!("Elemento de la tupla: {}", mi_tupla.1); // Imprime 6.5
    println!("Primer elemento del array: {}", mi_array[0]);
}