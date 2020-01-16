Strings

Un `String` es cualquier secuencia de caracteres encerrada entre comillas dobles ("").

Ejemplos


## Definición explícita

El siguiente código define explícitamente la variable usando la palabra clave `&str`:

```rust
fn definicion_explicita() {
    // definir explícitamente
    let lenguaje:&str = "Programación en Rust";

    println!("Valor: {}", lenguaje); 
}
```

## Definición implícita

El siguiente código define implícitamente el tipo de cadena de la variable al asignarles
el valor único entre comillas dobles.

```rust
fn definicion_implicita() { 
    // definir implícitamente
    let lenguaje_2 = "Programación en C++";

    println!("Valor: {}", lenguaje_2);
}
```