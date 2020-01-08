//! Las variables en `Rust` son immutables por defecto, es decir, que una vez que un valor 
//! está vinculado a un nombre no podemos cambiar ese valor. Este compartamiento se puede 
//! cambiar para hacer que las mismas sean mutables, es decir, que podamos cambiar de valor 
//! durante la ejecución del programa.

//! A continuación veremos cómo y por qué `Rust` nos alienta a utilizar la inmutabilidad.

// COMPILA CON ERROR!
fn main() {
    // Una variable se puede inicializar asignándole un valor cuando se declara. 
    // Se dice que el valor está vinculado a esa variable.

    let x = 5; // Inicializamos nuestra variable
    x = 6; // Esto no es posible, ya que la variable `x` es inmutable
}

//! Este programa nos mostrará un mensaje de error diciéndonos que no podemos 
//! asignar dos veces un valor a la variable `x`, ya que la misma es imnutable.

//! Algunas veces la mutabilidad puede ser muy útil. Es por esto que `Rust` cuenta con una palabra
//! llamada `mut`. Simplemente agregamos esta palabra delante del nombre de la variable.

//! Ahora modificaremos nuestro ejemplo para hacer que nuestra variable `x` sea mutable

// COMPILA SIN ERRORES!
fn ejemplo_de_mutabilidad() {

    let mut x = 5; // Ahora `x` es mutable
    x = 6; 
     
    x = 17;
    x = 34;
}

// Ahora podemos modificar tantas veces como deseamos el valor de nuestra variable utilizando `mut`.
// Si ejecutamos este código, el mismo no nos arrojará ningún error, sin embargo, si que nos mostrará
// 4 advertencias. 

// Esto debido a que `Rust` por defecto nos advierte sobre el código que hemos declarado, pero que no 
// hemos utilizado en nuestra aplicación, ya que en ningún momento estamos utilizando la variable `x`.
