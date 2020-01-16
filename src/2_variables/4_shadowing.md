El `shadowing` (sombreado) de variables es una técnica en la que una variable declarada dentro 
de un determinado alcance tiene el mismo nombre que una variable declarada en 
un alcance externo. Esto también se conoce como `masking` (enmascaramiento). 

Se dice que esta variable externa está sombreada por la variable interna, mientras 
que la variable interna oculta la variable externa.

El siguiente código explica este concepto:

```rust
fn main() {
  let variable_externa = 112;

  // Inicio del bloque de código
  { 
    let variable_interna = 213;
    println!("Variable de bloque: {}", variable_interna);

    let variable_externa = 117;
    println!("Variable de bloque exterior: {}", variable_externa);
  }
  // Fin del bloque de código

    println!("Variable externa: {}", variable_externa);
 }
 ```