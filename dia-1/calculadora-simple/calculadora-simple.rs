use std::io;

fn main() {
    println!("Introduce el primer número:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Fallo al leer la línea");
    let num1: f64 = num1.trim().parse().expect("Por favor, introduce un número");

    println!("Introduce el segundo número:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Fallo al leer la línea");
    let num2: f64 = num2.trim().parse().expect("Por favor, introduce un número");

    println!("Elige la operación (+, -, *, /):");
    let mut operacion = String::new();
    io::stdin().read_line(&mut operacion).expect("Fallo al leer la línea");

    let resultado = match operacion.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Operación no válida");
            return;
        }
    };

    println!("El resultado es: {}", resultado);
}