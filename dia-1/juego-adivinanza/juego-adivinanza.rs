use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el número!");

    let numero_secreto = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Por favor, introduce tu número.");

        let mut adivinanza = String::new();

        io::stdin()
            .read_line(&mut adivinanza)
            .expect("Fallo al leer la línea");

        let adivinanza: u32 = match adivinanza.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tu tu número: {}", adivinanza);

        match adivinanza.cmp(&numero_secreto) {
            Ordering::Less => println!("Demasiado pequeño!"),
            Ordering::Greater => println!("Demasiado grande!"),
            Ordering::Equal => {
                println!("¡Ganaste!");
                break;
            }
        }
    }
}
