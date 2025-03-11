use std::collections::HashMap;
use std::io;

struct Cuenta {
    saldo: f64,
}

impl Cuenta {
    fn new() -> Cuenta {
        Cuenta { saldo: 0.0 }
    }

    fn depositar(&mut self, cantidad: f64) {
        self.saldo += cantidad;
    }

    fn retirar(&mut self, cantidad: f64) -> Result<(), String> {
        if cantidad > self.saldo {
            return Err("Fondos insuficientes".to_string());
        }
        self.saldo -= cantidad;
        Ok(())
    }

    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }
}

fn main() {
    let mut cuentas: HashMap<u32, Cuenta> = HashMap::new();
    let mut contador = 0;

    loop {
        println!("Elige una opción: (1) Crear cuenta, (2) Depositar, (3) Retirar, (4) Consultar saldo, (5) Salir");

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Fallo al leer la línea");
        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match opcion {
            1 => {
                cuentas.insert(contador, Cuenta::new());
                println!("Cuenta {} creada.", contador);
                contador += 1;
            }
            2 => {
                println!("Introduce el ID de la cuenta:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Fallo al leer la línea");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Introduce la cantidad a depositar:");
                let mut cantidad = String::new();
                io::stdin().read_line(&mut cantidad).expect("Fallo al leer la línea");
                let cantidad: f64 = match cantidad.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if let Some(cuenta) = cuentas.get_mut(&id) {
                    cuenta.depositar(cantidad);
                    println!("Depósito realizado. Nuevo saldo: {}", cuenta.consultar_saldo());
                } else {
                    println!("Cuenta no encontrada");
                }
            }
            3 => {
                println!("Introduce el ID de la cuenta:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Fallo al leer la línea");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Introduce la cantidad a retirar:");
                let mut cantidad = String::new();
                io::stdin().read_line(&mut cantidad).expect("Fallo al leer la línea");
                let cantidad: f64 = match cantidad.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if let Some(cuenta) = cuentas.get_mut(&id) {
                    match cuenta.retirar(cantidad) {
                        Ok(_) => println!("Retiro realizado. Nuevo saldo: {}", cuenta.consultar_saldo()),
                        Err(mensaje) => println!("Error: {}", mensaje),
                    }
                } else {
                    println!("Cuenta no encontrada");
                }
            }
            4 => {
                println!("Introduce el ID de la cuenta:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Fallo al leer la línea");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if let Some(cuenta) = cuentas.get(&id) {
                    println!("Saldo de la cuenta {}: {}", id, cuenta.consultar_saldo());
                } else {
                    println!("Cuenta no encontrada");
                }
            }
            5 => break,
            _ => println!("Opción no válida"),
        }
    }
}