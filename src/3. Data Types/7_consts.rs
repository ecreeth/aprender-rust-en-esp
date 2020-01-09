//! Constantes

//! Las constantes son aquellas que se declaran en todo el alcance
//! del programa, lo que significa que su valor no puede modificarse.
//! Se pueden definir en el ámbito global y local.

//! Sintaxis

//! Se declaran usando la palabra clave `const` seguida del nombre,
//! dos puntos (:), y luego el tipo de datos de la variable.

//! Por convención, se escriben los nombres de las constantes
//! en SNAKE_CASE, es decir.

//! - Todas las letras deben ser mayúsculas.
//! - Todas las palabras deben separarse usando un guión bajo (_).

//! Ejemplos

//! El siguiente ejemplo define dos variables const:

//! - ID_1 en alcance global
//! - ID_2 en ámbito local

const NOMBRE: &str = "Luis M. Alvado"; // definir una constante global

fn main() {
    const ID_2: u32 = 3; // definir una constante local
    
    println!("ID: {}", ID);
    println!("ID 2: {}", ID_2);
}
