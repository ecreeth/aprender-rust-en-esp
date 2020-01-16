# Tuplas

Las tuplas son secuencias heterogéneas de elementos, lo que significa que cada
elemento de una tupla puede tener un tipo de datos diferente. Al igual que los
arrays, las tuplas tienen una longitud fija.

# Definir una tupla

Se puede definir una tupla escribiendo `let` seguido del nombre de la tupla y luego
encerrando los valores dentro del paréntesis.

### Sintaxis #1

La siguiente sintaxis define una tupla sin especificar el tipo. Sin embargo, el
compilador puede inferir el tipo.

```
let nombre_tupla = ("valor1", 'c', 1);
```

### Sintaxis #2

La siguiente sintaxis define una tupla especificando el tipo.

```
let nombre_tupla: (&str, char, i32) = ("valor1", 'c', 1);
```

Ejemplos

```rust
fn creacion_de_tuplas() {
    // definir un tupla
    let informacion_personal = ("eCreeth", 48, "35kg", "6ft");

    // define a tuple con sus tipos
    let informacion_personal: (&str, i32, &str, &str) = ("eCreeth", 48, "35kg", "6ft");
}
```

### Acceder al valor de la tupla

A diferencia de la matriz que usa [] para acceder a un elemento, se puede acceder
al valor de la tupla utilizando el operador de punto (.)

Para obtener los valores individuales de una tupla, podemos usar la coincidencia
de patrones para desestructurar un valor de tupla, como este:

```rust
fn acceder_a_sus_valores() {
    // definir una tupla
    let info_personal = ("Luis M. Alvarado", 48, "35kg", "6ft");

    // access value of a tuple
    println!("Nombre: {}, Edad: {}", info_personal.0, info_personal.1);

    // definir una tupla
    let info_personal_2 = ("Madelin", 48, "35kg", "6ft");

    // obtener valores individuales de la tupla
    let (nombre, edad, peso, altura) = info_personal_2;

    // Imprimir valores
    println!("Nombre: {}", nombre);
    println!("Edad: {}", edad);
    println!("Peso: {}", peso);
    println!("Altura: {}", altura);
}
```

### ¿Cómo hacer una tupla mutable?

Al igual que una variable se vuelve mutable al agregar la palabra clave `mut`
después de `let`, lo mismo ocurre con una tupla.

```rust
fn tuplas_mutables() {
    // definir una tupla
    let mut info_personal = ("Miguel", 48);

    // imprimir el valor de la tupla
    println!("Nombre: {}, Edad {}", info_personal.0, info_personal.1);

    // modificar el valor del indice 0
    info_personal.0 = "John";

    // imprimir el valor modificado
    println!("Nombre: {}, Edad {}", info_personal.0, info_personal.1);
}
```

### Imprimir Tuplas

Toda la tupla se puede mostrar utilizando el rasgo (trait) de depuración.

```rust
fn imprimir_tuplas() {
    // definir una tupla
    let info_personal = ("Alvarado", 23, "35kg", "6ft");

    // imprimir el valor de la tupla
    println!("Informacion Personal : {:?}", info_personal);
}
```
