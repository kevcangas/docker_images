// 1. DEFINICIÓN DEL TRAIT
trait Combatiente {
    // Retorna la vida restante después del golpe
    fn recibir_dano(&mut self, cantidad: i32) -> i32;
    
    // Retorna cuánto daño inflige al enemigo
    fn atacar(&mut self) -> i32;

    // Getter para saber la vida actual (solo lectura)
    fn vida_actual(&self) -> i32;
}

// ---------------------------------------------------------
// 2. ESTRUCTURA 1: Guerrero (Usa Armadura)
// ---------------------------------------------------------
struct Guerrero {
    hp: i32,
    ataque: i32,
    defensa: i32, // Reducción de daño
}

impl Guerrero {
    fn new(hp: i32, ataque: i32, defensa: i32) -> Self {
        Guerrero { hp, ataque, defensa }
    }
}

impl Combatiente for Guerrero {
    fn vida_actual(&self) -> i32 {
        self.hp
    }

    fn atacar(&mut self) -> i32 {
        // El guerrero es simple, siempre golpea con su fuerza total.
        self.ataque
    }

    fn recibir_dano(&mut self, cantidad: i32) -> i32 {
        // TODO:
        // 1. Calcular el daño real: (cantidad - self.defensa).
        //    OJO: Si la defensa es mayor al daño (ej: daño 5, defensa 10), 
        //    el daño real debe ser 0, no negativo (usar .max(0)).
        // 2. Restar el daño real a self.hp.
        // 3. Retornar self.hp.
        
        let dano_real = (cantidad - self.defensa).max(0); // <-- Corregir esto
        self.hp -= dano_real;
        self.hp
    }
}

// ---------------------------------------------------------
// 3. ESTRUCTURA 2: Mago (Usa Maná)
// ---------------------------------------------------------
struct Mago {
    hp: i32,
    poder_magico: i32,
    mana: i32,
}

impl Mago {
    fn new(hp: i32, poder_magico: i32, mana: i32) -> Self {
        Mago { hp, poder_magico, mana }
    }
}

impl Combatiente for Mago {
    fn vida_actual(&self) -> i32 {
        self.hp
    }

    fn recibir_dano(&mut self, cantidad: i32) -> i32 {
        // El mago no tiene armadura, recibe el daño completo.
        self.hp -= cantidad;
        self.hp
    }

    fn atacar(&mut self) -> i32 {
        // TODO:
        // 1. Verificar si tiene al menos 10 de mana.
        // 2. SI TIENE MANA: 
        //    - Restar 10 de mana.
        //    - Retornar self.poder_magico.
        // 3. NO TIENE MANA:
        //    - Retornar solo 2 de daño (golpe débil).
        
        if self.mana >= 10 {
            self.mana -= 10;
            self.poder_magico
        } else {
            2
        }
    }
}

// Función main para probar manualmente si quieres
fn main() {
    let mut g = Guerrero::new(100, 15, 5);
    let mut m = Mago::new(50, 30, 20);

    println!("Guerrero ataca: {}", g.atacar());
    println!("Mago ataca con maná: {}", m.atacar());
    println!("Mago ataca sin maná: {}", m.atacar()); // Debería gastarse el maná en el anterior
}

// ---------------------------------------------------------
// 4. PRUEBAS UNITARIAS (NO TOCAR, SOLO EJECUTAR)
// Si usas 'cargo test' o el botón de Run Test en tu IDE
// ---------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guerrero_reduce_dano() {
        // Guerrero con 100 HP y 5 de defensa
        let mut g = Guerrero::new(100, 10, 5);
        
        // Recibe 20 de daño. 20 - 5 defensa = 15 daño real.
        // Vida restante debería ser 85.
        assert_eq!(g.recibir_dano(20), 85); 
    }

    #[test]
    fn test_guerrero_tanque_total() {
        // Guerrero con mucha defensa (10)
        let mut g = Guerrero::new(100, 10, 10);
        
        // Recibe 5 de daño. 5 - 10 = -5. No debería curarse, debería ser 0 daño.
        // Vida se mantiene en 100.
        assert_eq!(g.recibir_dano(5), 100);
    }

    #[test]
    fn test_mago_gasta_mana() {
        // Mago con 20 de mana. Costo hechizo = 10.
        let mut m = Mago::new(50, 40, 20);
        
        // Primer ataque: Tiene mana (20), hace daño completo (40)
        assert_eq!(m.atacar(), 40);
        
        // Segundo ataque: Tiene mana (10 restantes), hace daño completo (40)
        assert_eq!(m.atacar(), 40);
        
        // Tercer ataque: No tiene mana (0 restantes), hace daño débil (2)
        assert_eq!(m.atacar(), 2);
    }
}