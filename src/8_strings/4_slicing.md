# Slicing(Cortando) un String

El `slicing` se usa para obtener una parte de un valor de cadena.

## Sintaxis

```rust
let slice = &string[start_index..end_index]
```

Aquí,

     - `start_index` y `end_index` son las posiciones del índice inicial y final de la matriz original, respectivamente.

> Nota: el índice_inicio es inclusivo y el índice final es exclusivo

     - `&` indica que la porción variable toma prestada la cadena.

> Nota: El valor mínimo de start_index es 0 y el valor máximo es el tamaño de la cadena.

```rust
fn main() {
   let string = "Rust Programming".to_string();
   let slice = &string[5..12]; 
   // get characters at 5,6,7,8,9,10 and 11 indexes
   println!("Slice : {}", slice);
}
```