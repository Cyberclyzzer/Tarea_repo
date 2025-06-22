use std::io;

//Funcion que devuelve el cuadrado de un numero
//Crear un struct de Libro, con sus atributos, y ademas un impl que tenga la funcion resume().

struct Libro {
    titulo: String,
    autor: String,
    año: u32,
}

impl Libro {
    fn resume(&self) {
        println!("Título: {}, Autor: {}, Año: {}", self.titulo, self.autor, self.año);
    }
}

fn main() {
    let mut entrada_str = String::new(); // Declarar una String mutable
    println!("Ingrese el numero al cual elevar (Que sea entero por favor)");
    io::stdin().read_line(&mut entrada_str).expect("Error al leer la entrada"); // Pasar la String mutable como argumento
    let numero = entrada_str.trim().parse::<i32>().expect("Por favor, ingresa un número entero");
    let resultado_cuadrado = cuadrado(numero);
    println!("El cuadrado de {} es {}", numero, resultado_cuadrado);

    let mi_libro = Libro {
        titulo: String::from("Mil Años de Seriedad"),
        autor: String::from("Jesus Silva"),
        año: 1954,
    };

    mi_libro.resume();
}

fn cuadrado(numero: i32) -> i32 {
    numero * numero
}