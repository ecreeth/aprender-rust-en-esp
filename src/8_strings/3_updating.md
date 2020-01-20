# Actualización de Strings

Una cadena existente se puede actualizar agregando un carácter o una cadena.

> 💡 ¿Por qué no hacer una nueva cadena en lugar de actualizar una existente?
La actualización de una Cadena existente es útil cuando desea realizar cambios en una Cadena existente
en tiempo de ejecución en lugar de compilar una como, en situaciones donde se realizan cambios en la 
Cadena en una condición.

# Insertar un solo carácter

Hay casos en los que se requiere actualizar una cadena presionando un solo carácter. Un ejemplo es crear una
cadena que contenga un solo carácter repetido N veces en una condición particular. Rust te ayuda a hacerlo
mediante el método push.

Pasos para insertar un caracter a un `String`:

     - Hacer una variable de cadena mutable.
     - Para insertar un solo carácter Unicode en un objeto String, pase un carácter dentro del método incorporado `push()`.

¡El siguiente código muestra cómo hacerlo!

```rust
fn main() {
  // define a String object
  let mut course = String::from("Rus");
  // push a character
  course.push('t');
  println!("This is a beginner course in {}.", course);
}
```

Hay casos en que se requiere hacer crecer una Cadena concatenando una nueva Cadena a una Cadena existente.
¡Rust lo ayuda a hacerlo utilizando el operador push, + y el formato! método macro

# Empuje una cadena

Rust te ayuda a hacer crecer un objeto String usando un método `push_str`.

Pasos para empujar una cadena a una cadena:

     - Hacer una variable de cadena mutable.
     - Para insertar una cadena en una variable de cadena ampliable, pase un carácter dentro del método incorporado push_str().

¡El siguiente código muestra cómo hacerlo!

```rust
fn main() {
  // define a string object
  let mut course = String::from("Rust");
  // push a string
  course.push_str(" Programming");
  println!("This is a beginner course in {}.", course);
}
```

# Concatenación con el operador (+)

Una cadena se puede concatenar a otra cadena usando el operador `+`.

> Nota: El operando del lado derecho es prestado mientras se concatena usando el operador +.

¡El siguiente código muestra cómo hacerlo!

```rust
#[allow(unused_variables, unused_mut)]
fn main(){
   // define a String object 
   let course = "Rust".to_string();
   // define a String object
   let course_type = " beginner course".to_string();
   // concatenate using the + operator
   let result = course + &course_type;
   println!("{}", result);
}
```

# Formato macro

Para agregar dos o más objetos String juntos, hay una macro llamada `format!`. Toma variables o valores y los fusiona en una sola cadena.

> Nota: La macro `format!` permite concatenar en el orden deseado al pasar un número entero positivo dentro del marcador de posición. Si no se menciona el número, se concatenará en el orden de los valores escritos.

Para mostrar el resultado del format! macro, el resultado se debe guardar en una variable.

¡El siguiente código muestra cómo hacerlo!

```rust
fn main(){
   let course = "Rust".to_string();
   let _course_type = "beginner course".to_string();
   // default format macro 
   let result = format!("{} {}", course, _course_type);
   // passing value in the placeholder in the format macro 
   let result = format!("{1} {0}", course,_course_type);
   println!("{}", result);
}
```