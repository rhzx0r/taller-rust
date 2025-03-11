use std::collections::HashMap;
use std::io;

fn main() {
    let mut tareas = HashMap::new();
    let mut contador = 0;

    loop {
        println!("Elige una opción: (1) Agregar tarea, (2) Listar tareas, (3) Eliminar tarea, (4) Salir");

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Fallo al leer la línea");
        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match opcion {
            1 => {
                println!("Introduce la descripción de la tarea:");
                let mut descripcion = String::new();
                io::stdin().read_line(&mut descripcion).expect("Fallo al leer la línea");
                tareas.insert(contador, descripcion.trim().to_string());
                contador += 1;
            }
            2 => {
                for (id, tarea) in &tareas {
                    println!("{}: {}", id, tarea);
                }
            }
            3 => {
                println!("Introduce el ID de la tarea a eliminar:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Fallo al leer la línea");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                tareas.remove(&id);
            }
            4 => break,
            _ => println!("Opción no válida"),
        }
    }
}