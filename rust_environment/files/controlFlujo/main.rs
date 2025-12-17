fn main() {
    // Usamos un rango del 1 al 5 (inclusive el 5)
    for numero in 1..=5 {
        // Estructura if-else básica
        if numero % 2 == 0 {
            println!("El número {} es PAR", numero);
        } else {
            println!("El número {} es IMPAR", numero);
        }
    }

    println!("--- Probando loop con break ---");
    
    let mut contador = 0;
    // Bucle infinito con salida manual
    loop {
        contador += 1;
        if contador == 3 {
            println!("Llegamos a 3, saliendo del loop...");
            break;
        }
    }
}