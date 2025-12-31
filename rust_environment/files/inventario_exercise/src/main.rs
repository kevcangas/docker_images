use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Product {
    description: String,
    price: f64,
    stock: u32
}

struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: String, description: String, price: f64, stock: u32) {
        // TODO: Implementar lógica. 
        // Tip: Revisa .entry() o .insert() en la documentación de HashMap
        let new_product = Product{
            description, 
            price, 
            stock,
        };
        self.products.entry(name).or_insert(new_product);
    }

    fn edit_price(&mut self, name: &str, new_price: f64) -> Result<(), String> {
        // TODO: Buscar el producto. Si existe, actualizar precio. 
        // Si no, retornar Err("Producto no encontrado".to_string())
        if let Some(product) = self.products.get_mut(name) {
            product.price = new_price; // Al estar dentro del Some, ya tenemos acceso mutable
            Ok(())
        } else {
            Err("Producto no encontrado".to_string())
        }
    }

    fn sell_product(&mut self, name: &str, quantity: u32) -> Result<(), String> {
        // TODO: 
        // 1. Verificar si existe.
        // 2. Verificar si hay suficiente stock.
        // 3. Restar stock o retornar error.
        if let Some(product) = self.products.get_mut(name) {
            if product.stock >= quantity {
                product.stock -= quantity;
                Ok(())
            }
            else{
                Err("Stock insuficiente".to_string())
            }
        }
        else{        
            Err("Producto no encontrado".to_string())
        }
    }

    fn list_products(&self) {
        println!("\n--- Inventario Actual ---");
        // TODO: Iterar sobre self.products e imprimir
        for product in &self.products {
            println!("Product: {:?}", product)
        }
    }

}

fn main() {
    let mut store = Inventory::new();

    // Puedes simular la entrada de usuario o hardcodear pruebas aquí:
    store.add_product("Laptop".to_string(), "Gaming Laptop".to_string(), 1500.00, 5);
    store.add_product("Mouse".to_string(), "Wireless Mouse".to_string(), 25.50, 50);

    // Intenta vender
    match store.sell_product("Laptop", 2) {
        Ok(_) => println!("Venta exitosa"),
        Err(e) => println!("Error en venta: {}", e),
    }

    // Intenta vender algo que no existe o sin stock para probar errores
    match store.sell_product("Laptop", 10) {
        Ok(_) => println!("Venta exitosa"),
        Err(e) => println!("Error esperado: {}", e),
    }

    store.list_products();


}
// ... Todo tu código del Inventory y main arriba ...

#[cfg(test)]
mod tests {
    use super::*; // Esto importa todo lo que está fuera del módulo 'tests' (tu struct Inventory, Product, etc.)

    #[test]
    fn test_add_product() {
        let mut store = Inventory::new();
        store.add_product("Laptop".to_string(), "Gaming".to_string(), 1000.0, 10);

        // Verificamos que se haya guardado (asumiendo que puedes acceder al HashMap)
        // Nota: Para que esto funcione, asegúrate que 'products' sea accesible o usa un método getter.
        // Si estamos en el mismo archivo, Rust permite acceder a campos privados en tests.
        assert!(store.products.contains_key("Laptop"));
        assert_eq!(store.products.get("Laptop").unwrap().stock, 10);
    }

    #[test]
    fn test_sell_product_success() {
        let mut store = Inventory::new();
        store.add_product("Mouse".to_string(), "Wireless".to_string(), 20.0, 50);

        // Vendemos 5
        let result = store.sell_product("Mouse", 5);
        
        // 1. Verificamos que el resultado sea Ok(())
        assert!(result.is_ok());
        
        // 2. Verificamos que el stock bajó a 45
        assert_eq!(store.products.get("Mouse").unwrap().stock, 45);
    }

    #[test]
    fn test_sell_product_insufficient_stock() {
        let mut store = Inventory::new();
        store.add_product("Teclado".to_string(), "Mecánico".to_string(), 100.0, 2);

        // Intentamos vender 5 (solo hay 2)
        let result = store.sell_product("Teclado", 5);

        // Debe dar error
        assert!(result.is_err());
        
        // Opcional: Verificar el mensaje de error exacto
        assert_eq!(result.unwrap_err(), "Stock insuficiente"); // Ajusta el mensaje al que tú escribiste
    }

    #[test]
    fn test_edit_price() {
        let mut store = Inventory::new();
        store.add_product("Monitor".to_string(), "4K".to_string(), 300.0, 5);

        // Editamos precio
        let _ = store.edit_price("Monitor", 250.0);

        assert_eq!(store.products.get("Monitor").unwrap().price, 250.0);
    }
}