
// Las variables en `Rust` son immutables por defecto; Este compartamiento se puede 
// cambiar para hacer que las mismas sean mutables, es decir, que podamos cambiar de 
// valor a la mismo durante la ejecucion del programa.

// Acontinuacion veremos cómo y por qué `Rust` nos alienta a utilizar la inmutabilidad.

fn main() {
  let x = 5;
println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}

// Cuando una variable es inmutable, una vez que un valor está vinculado a un nombre, 
// no podemos cambiar ese valor. 