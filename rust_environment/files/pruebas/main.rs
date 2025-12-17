fn main() {
    // --- 1. PRÉSTAMO INMUTABLE (&) ---
    // Queremos calcular el largo del texto sin perderlo
    let s1 = String::from("Hola Rust");

    // Pasamos &s1 (una referencia/préstamo), NO s1
    let len = calcular_largo(&s1);

    // ¡Magia! s1 sigue siendo válido aquí porque solo lo "prestamos"
    println!("El texto '{}' tiene una longitud de {}.", s1, len);


    // --- 2. PRÉSTAMO MUTABLE (&mut) ---
    // Para que alguien modifique tu dato prestado, la variable debe ser mut
    let mut s2 = String::from("Hola");

    // Pasamos &mut s2. Le damos permiso explícito para cambiarlo.
    agregar_mundo(&mut s2);

    // s2 ha sido modificado
    println!("Texto modificado: {}", s2);

    {
    let b = &s2;
    println!("{}",b);
    }

    {
    let a = &mut s2;
    println!("{}",a);
    }

    let c = 5;

    let d = c + 5;

    d = c * 5;

    println!("{}",d)

}

// Recibe &String (una referencia). 
// NO puede modificar el texto, solo leerlo.
fn calcular_largo(s: &String) -> usize {
    s.len() // No hace falta return, es la última expresión
} // Aquí s sale del ámbito, pero como es prestado, no se borra nada.

// Recibe &mut String (referencia mutable).
// SÍ puede modificar el texto.
fn agregar_mundo(s: &mut String) {
    s.push_str(" Mundo");
}