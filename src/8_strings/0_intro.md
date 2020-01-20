# Introducción a los Strings

Los `Strings` son una secuencia de caracteres Unicode. En `Rust`, una cadena no tiene terminación nula
a diferencia de los `Strings` en otros lenguajes de programación. Pueden contener caracteres nulos.

# Tipos de cadenas

Las cadenas son de dos tipos: `&str` y `String`

## Slice de String (&str)

Un `slice` de `String` tiene las siguientes propiedades:

     - Tipo primitivo
     - Inmutable
     - Cadena de longitud fija almacenada en algún lugar de la memoria
     - El valor de la cadena se conoce en tiempo de compilación

> Nota: Un `slice` de un `String` también se conoce como un segmento de cadena.

```rust
fn main() {
  // definir un slice de un String
  let lenguaje: &str = "Rust";

  // imprimir el slice String
  println!("Slice de String: {}", lenguaje);

  // imprimir la longitud del slice
  println!("La longitud es: {}", lenguaje.len());
}
```

## Objeto String

Un objeto `String` tiene las siguientes propiedades:

     - Una cadena está codificada como una secuencia UTF-8
     - Estructura de datos asignada al montón
     - El tamaño de esta cadena se puede modificar.
     - Sin terminación nula
     - Codifique los valores de cadena que se proporcionan en el tiempo de ejecución

### Crear un objeto tipo String

Hay muchas formas diferentes de crear y manipular objetos de cadena. Discutiremos dos aquí.

#### Crear un objeto String vacío

Este método convierte la cadena vacía o un literal de cadena en un objeto de cadena usando el método `.tostring()`.

```rust
fn main() {
  // crear un String vacío
  let course1 = String::new();

  // convert String literal to String object using .to_string
  let s_course1 = course1.to_string();

  // print the String object
  println!("Este es un String vacío{}.", s_course1);

  // print the length of an empty String  object
  println!("This is a length of my empty string {}.", s_course1.len());

  // create a String literal
  let course2 = "Rust Programming";

  // convert String literal to string object using .to_string
  let s_course2 = course2.to_string();

  // print the String object
  println!("This is a string literal : {}.", s_course2);

  // print the length of a String object
  println!("This is a length of my string literal {}.", s_course2.len());

  // define a String object using from method
  let course3 = String::from("Rust Language");

  // print the String object
  println!("This is a string literal : {}.", course3);

  // print the length of an string object
  println!("This is the length of my string literal {}.", course3.len());
}

> Nota: `len()` es una función incorporada que se usa para encontrar la longitud de un objeto String literal y String.