use std::fmt::Display;

// 1. DEFINICIÓN DEL TRAIT
// Este es el "contrato". Cualquier cosa que quiera ser un método de pago
// debe tener la capacidad de intentar un pago.
trait PaymentMethod: Display {
    // Debe retornar Ok(mensaje de éxito) o Err(mensaje de fallo)
    // &mut self es necesario porque el pago modifica el saldo/cupo.
    fn charge(&mut self, amount: f64) -> Result<String, String>;
}

// ---------------------------------------------------------
// 2. ESTRUCTURA 1: Tarjeta de Crédito
// Funciona con "límite de crédito".
// ---------------------------------------------------------
#[derive(Debug)]
struct CreditCard {
    owner: String,
    limit: f64,
    balance_used: f64,
}

impl CreditCard {
    fn new(owner: &str, limit: f64) -> Self {
        CreditCard {
            owner: owner.to_string(),
            limit,
            balance_used: 0.0,
        }
    }
}

// Implementamos Display para que se vea bonito al imprimir
impl Display for CreditCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tarjeta de {} (Disp: ${:.2})", self.owner, self.limit - self.balance_used)
    }
}

// AQUÍ ESTÁ EL RETO: Implementar el Trait para CreditCard
impl PaymentMethod for CreditCard {
    fn charge(&mut self, amount: f64) -> Result<String, String> {
        // TODO:
        // 1. Verificar si (balance_used + amount) supera el 'limit'.
        // 2. Si lo supera, retornar Err("Límite excedido").
        // 3. Si no, sumar amount a balance_used y retornar Ok.
        if self.balance_used + amount > self.limit {
            Err("Limite excedido".to_string())
        }
        else{
            self.balance_used += amount;
            Ok("Operación correcta".to_string())
        }
    }
}

// ---------------------------------------------------------
// 3. ESTRUCTURA 2: Billetera Bitcoin
// Funciona con "saldo disponible" (no crédito).
// ---------------------------------------------------------
#[derive(Debug)]
struct BitcoinWallet {
    address: String,
    btc_balance: f64,
    btc_price_in_usd: f64, // Simple conversión para el ejercicio
}

impl BitcoinWallet {
    fn new(address: &str, btc: f64) -> Self {
        BitcoinWallet {
            address: address.to_string(),
            btc_balance: btc,
            btc_price_in_usd: 50000.0, // Precio fijo para el ejercicio
        }
    }
}

impl Display for BitcoinWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wallet {} (BTC: {:.4})", self.address, self.btc_balance)
    }
}

// AQUÍ ESTÁ EL RETO: Implementar el Trait para BitcoinWallet
impl PaymentMethod for BitcoinWallet {
    fn charge(&mut self, amount: f64) -> Result<String, String> {
        // TODO:
        // 1. Convertir el monto en USD a BTC (amount / self.btc_price_in_usd).
        // 2. Verificar si hay suficiente btc_balance.
        // 3. Restar y retornar Ok, o retornar error.
        if self.btc_balance > (amount / self.btc_price_in_usd){
            self.btc_balance -= amount / self.btc_price_in_usd;
            Ok("Operación correcta".to_string())
        }
        else {
            Err("No hay suficiente dinero".to_string())
        }
    }
}

// ---------------------------------------------------------
// 4. FUNCIÓN GENÉRICA (POLIMORFISMO)
// Esta función acepta CUALQUIER cosa que implemente PaymentMethod
// ---------------------------------------------------------
fn process_transaction<T: PaymentMethod>(method: &mut T, amount: f64) {
    println!("Intentando cobrar ${} a: {}", amount, method);
    
    match method.charge(amount) {
        Ok(msg) => println!("✅ Éxito: {}", msg),
        Err(e) => println!("❌ Error: {}", e),
    }
    println!("---");
}

fn main() {
    let mut my_card = CreditCard::new("Juan Pérez", 1000.0);
    let mut my_crypto = BitcoinWallet::new("1A1zP1...", 0.05); // 0.05 BTC * 50000 = $2500

    // Caso 1: Compra con tarjeta (Exitosa)
    process_transaction(&mut my_card, 200.0);

    // Caso 2: Compra con tarjeta (Fallida por límite)
    process_transaction(&mut my_card, 900.0); // 200 + 900 > 1000

    // Caso 3: Compra con Crypto (Exitosa)
    process_transaction(&mut my_crypto, 1000.0);
    
    // Caso 4: Compra con Crypto (Fallida por saldo)
    process_transaction(&mut my_crypto, 5000.0);
}
