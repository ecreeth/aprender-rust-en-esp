# Introducción a las funciones

Una función es un bloque de código que se puede reutilizar. Se utiliza
para realizar algunas tareas específicas.

## Función Principal

La función más simple posible que hemos estudiado hasta ahora es la función
principal que se declara con la palabra clave fn. Aquí es donde comienza
la ejecución del programa.

Sin embargo, es posible definir una función definida por el usuario.

## Funciones definidas por el usuario

Las funciones personalizadas y escritas por el programador para realizar
las tareas especificadas.

## Definir una función

Una función se declara con la palabra clave `fn`.

Convención de nomenclatura: la convención para escribir el nombre de una función
está en un caso de serpiente, es decir,

	- Todas las letras deben estar en minúscula.
	- Todas las palabras deben estar separadas por guiones bajos

## Llamar a una función

La función se ejecuta cuando se invoca.

### Sintaxis

La expresión comienza con el nombre de la función seguido de los corchetes y
luego el punto y coma. Los parámetros de la función se pueden agregar entre
paréntesis si es necesario.

La sintaxis general para invocar una función:

📝 Nota: Se puede invocar una función definida por el usuario desde otra función o
desde la función principal. Se puede definir en cualquier lugar, encima o
debajo de la función principal.

Si se invoca una función pero su definición no existe, el compilador arrojará un error
de compilación, ❌, como el error [E0425]: no se puede encontrar la función
function_xyz en este ámbito.

📎 Ejemplo:

El siguiente ejemplo crea una función definida por el usuario y la invoca
desde la función principal.

```rust
// declarar una función
fn mostrar_mensaje(){
  println!("Hola, desde mi primera función");
}

// función principal
fn main() {
  // invocar la función
  mostrar_mensaje();
  println!("Fin de la función principal");
}
```