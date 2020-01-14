//! La palabra clave "use"

//! Utilizar la palabra clave `use`

//! Rust le ayuda a realizar una llamada de módulo precisa utilizando la palabra clave `use`.

//! ❓: ¿Por qué usar la palabra clave `use`?

//! El beneficio es mayor cuando los elementos en el mismo módulo deben mencionarse en el
//! código una y otra vez. Ahora, no tenemos que escribir todo el camino una y otra vez.

//! 📎 Ejemplo

//! Recordemos el ejemplo de la lección anterior. El siguiente ejemplo muestra cómo podemos
//! escribir un código preciso usando una palabra clave use:

pub mod capitulo {
	// mod nivel 1
  pub mod leccion {
  	// mod nivel 2
    pub mod titulo {
    	// mod nivel 3
      pub fn ilustracion() {
        println!("Hola, soy una función anidada de tercer nivel");
      }
    }
  }
}

// hacer uso de la palabra clave `use`
use capitulo::leccion::titulo;

fn main() {
  titulo::ilustracion(); // llamar la función
}

//! # Operador Glob (*)

//! El operador global le ayuda a evitar escribir `EnumDias::domingo` al asignar un valor `enum` a una variable.

//! 📎 Ejemplo

//! Recuerde el ejemplo en la lección Introducción a las enumeraciones. El siguiente ejemplo muestra
//! cómo podemos evitar escribir un código extenso usando el operador global (*).

// hacer esta `enum` printiable utilizando `fmt::Debug`.
#[derive(Debug)]
enum HacerMovimiento{
   Horizontal,
   Vertical
}

use HacerMovimiento::*; // utilizar el operador global

fn main() {

   // utilizar las enumeraciones
   let mov_horizontal = Horizontal; // `Horizontal` es un atajo para `HacerMovimiento::Horizontal`

   let mov_vertical = Vertical; // `Vertical` es un atajo para `HacerMovimiento::Vertical`

   // imprimir las enumeraciones
   println!("Valor: {:?}", mov_horizontal);
   println!("Valor: {:?}", mov_vertical);

}