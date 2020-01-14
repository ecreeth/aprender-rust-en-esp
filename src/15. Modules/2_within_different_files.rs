//! Cuando los módulos se hacen más grandes y se vuelven engorrosos de almacenar en un
//! solo archivo, es posible mover sus definiciones a un archivo separado para facilitar
//! la navegación del código. Es posible acceder a un módulo incluso si pertenece a un
//! archivo diferente. Para usar el módulo en un archivo diferente, escriba `mod` seguido
//! del nombre del archivo en el que se declara el módulo.

// ------------------------------------ 

// # Declaración implícita

// Un bloque de código colocado en un archivo sin envolverlo en un bloque `mod`
// declara implícitamente un módulo.

// 📎 Importar el módulo:  mod nombre_archivo

// 📎 Llamar al módulo` :  nombre_archivo::x

// Donde `x` puede ser cualquier construcción dentro del módulo, es decir, una función,
// matriz, trait, estructura.

// ------------------------------------

// # Declaración explícita

// El código en un archivo está envuelto dentro del bloque `mod`.
// Esto declara explícitamente un módulo.

// 📎 Importar el módulo:  mod nombre_archivo

// 📎 Llamar al módulo` :   nombre_archivo::nombre_modulo::x

// Donde `x` puede ser cualquier construcción dentro del módulo, es decir, una función,
// matriz, trait, estructura.

// ------------------------------------

// # Regla de privacidad

	// - Si el módulo que pertenece a algún otro archivo se debe hacer accesible, entonces
	//   debe hacerse público utilizando la palabra clave `pub` antes del `mod`.

// 📝 Nota: Una vez que el módulo se hace público mediante `pub`, se aplican todas las reglas de
// privacidad para definir módulos dentro del mismo archivo.

// ------------------------------------

// # Ejemplo

// El siguiente ejemplo muestra cómo se puede acceder a un módulo en otro archivo.

// 📎 Declaración implícita

// El siguiente ejemplo declara un módulo implícitamente en un archivo mi_modulo.rs y lo llama desde main.rs.

// 📝 Nota: los módulos de declaración implícita son públicos de forma predeterminada

// archivo `main.rs`
mod mi_modulo; 

fn main() {

  println!("Invocar función de mi_modulo.rs");

  mi_modulo::mi_funcion_publica();

}

// archivo `mi_modulo.rs`
pub fn mi_funcion_publica() {
    println!("Soy un función pública en mi_modulo.rs");
}

// ------------------------------------

// # Declaración explícita

// El siguiente ejemplo declara un módulo de módulo en un archivo mi_modulo.rs y lo llama desde main.rs.

// archivo `mi_modulo.rs`
pub mod cuentas {
	pub fn dinero_disponible() {
	    println!("Soy un función pública en mi_modulo.rs");
	}
}

// archivo main.rs
mod mi_modulo;

fn main() {
  println!("Soy un función pública en mi_modulo.rs");

  mi_modulo::cuentas::dinero_disponible();
}