# Asignación de múltiples variables

Es posible asignar múltiples variables en una declaración.

```rust
fn main() {
	// Declaramos las variables `curso` y `nivel` 
    let (curso, nivel) = ("Rust", "básica");

	Equivalente a:

	// let curso = "Rust";
	// let nivel = "básica";

	// Imprimimos las variables
    println!("Introducción {} al lenguaje {}.", nivel, curso);
}
```

# Asignación de múltiples variables utilizando la mutibilidad

```rust
fn main() {
	// Declaramos las variables `curso` y `nivel` y las hacemos mutables 
    let (mut curso, mut nivel) = ("Rust", "básica");

	// Imprimimos las variables
    println!("Introducción {} al lenguaje {}.", nivel, curso);

    // Cambiamos su valor
    curso = "C++";
    nivel = "Avanzada";

    // Reimprimimos las variables
    println!("Introducción {} al lenguaje {}.", nivel, curso);
}
```

Cuando asignamos múltiples variables en un misma declaración y necesitamos
mutabilidad, debemos agregar la palabra clave `mut` en aquellas variables
que serán mutables. 