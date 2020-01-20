# Iterando sobre Strings

Los siguientes métodos describen tres formas diferentes de atravesar una cadena:

### Tokenizar un objeto tipo String

Un objeto String puede ser tokenizado en espacios en blanco o un token de personaje.

### Tokenizing para separar en espacios en blanco

`split_whitespace` se usa para dividir una cadena en la aparición de espacios en blanco. Recorra la cadena
para dividir en espacios en blanco usando un bucle `for`.

## Sintaxis

```rust
for found in  str.split_whitespace(){
    println!("{}", found);
}
```

Aquí `str` es la Cadena original que se debe atravesar, `split_whitespace()` es una palabra clave incorporada
para dividir una cadena en espacios en blanco, ya que se utiliza para recorrer la Cadena e imprimirla tan pronto
como se encuentre y se encuentre el espacio en blanco es un iterador sobre la cadena.

```rust
fn main() {
  // define a String object
  let str = String::from("Rust Programming"); 
  // split on whitespace
  for token in str.split_whitespace(){
      println!("{}", token);
  }
}
```

# Tokenizing para dividir en un carácter personalizado

El método de división se utiliza para dividir una oración en algún token. El token se especifica en el método de división.
Esto sería útil para procesar datos separados por comas, que es una tarea de programación común.

## Sintaxis

```rust
for found in str.split(","){
    println!("{}", found);
}
```

Aquí str es el String original que se debe atravesar, str.split () es un método incorporado que toma un parámetro, es decir,
cualquier delimitador y divide la oración en ese parámetro, ya que se usa para recorrer el String e imprimir una palabra antes de la ficha.

```rust
fn main() {
  // define a String object
  let str = String::from("Educ, course on, Rust, Programming");  
  // split on token
  for token in str.split(","){
      println!("{}", token);
  }
}
```

# Iterando sobre el objeto de cadena

El método `chars` permite iterar sobre cada elemento en una cadena utilizando un bucle `for`.

## Sintaxis

```rust
for found in  str.chars(){
    println!("{}", found);
}
```

Aquí str es la cadena original que se debe atravesar, `str.chars()` es una palabra clave incorporada para
denotar letras en una cadena, ya que se usa para recorrer la cadena e imprimir cada literal, y se encuentra
un iterador sobre un `String`.

```rust
fn main() {
  // define a String object
  let str = String::from("Rust Programming");  
  // split on literal
  for token in str.chars(){
      println!("{}", token);
  }
}
```