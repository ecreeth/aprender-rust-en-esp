# Métodos básicos del objeto String

Algunos de los métodos principales se analizan en esta lección. Puede encontrar una lista de todos los métodos
de String en la documentación de [Rust de Strings](https://doc.rust-lang.org/std/string/struct.String.html).

# Capacidad en bytes

La capacidad da el número de bytes asignados a un `String`, a diferencia de `len()`, que da el número de bytes
tomados por el objeto Cadena. Para obtener la capacidad de una variable en bytes, use la función incorporada `capacity()`.

### Sintaxis

La sintaxis general es: `str.capacity();`

Aquí `str` es la cadena cuya capacidad se encuentra.

> Nota: La longitud de la cadena siempre será menor o igual que la capacidad.

```rust
fn main() {
  // define a growable string variable
  let course = String::from("Rust");

  println!("This is a beginner course in {}.", course);

  //capacity in bytes
  println!("Capacity: {}.", course.capacity());
}
```

# Encontrar una subcadena

Para encontrar si una cadena contiene otra cadena, use la función incorporada `contains()`.

### Sintaxis

La sintaxis general es: `str.contains("sub_str")`

Aquí `str` es la cadena original y `"sub_str"` es una subcadena que se encuentra en una cadena.

```rust
fn main() {
  // define a growable string variable
  let str = String::from("Rust Programming"); 
  let sub_str = String::from("Rust"); 
  println!("This is a beginner course in {}.", str);
  // find if string contains a substring
  println!("{} is a substring of {}: {}.", sub_str, str, str.contains("Rust"));
}
```

# Reemplazar una subcadena

Para reemplazar todas las apariciones de una subcadena dentro de un objeto `String` con otra cadena,
use la función incorporada `replace()`.

## Sintaxis

La sintaxis general es: `str.replace(replace_from, replace_to)`

Aquí `str` es la cadena original, `replace_from` es el valor que se va a reemplazar en la cadena `str`
y `replace_to` es el valor al que se convierte la cadena.

```rust
fn main() {
  // define a growable string variable
  let str = String::from("Rust Programming"); 
  let replace_from = "Programming";
  let replace_to = "Language"; 
  // find if string contains a substring
  let result = str.replace(replace_from, replace_to);
  println!("{} now becomes {}.", str, result);
}
```

# Recortar una cadena

Para recortar una cadena, use la función `trim()`. Se utiliza para eliminar espacios en blanco iniciales y
finales en una cadena.

## Sintaxis

La sintaxis general es: `str.trim()`

> Nota: La función de recorte no elimina el espacio entre la cadena.