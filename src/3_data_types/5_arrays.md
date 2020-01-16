# Arrays

Un array es una secuencia homogénea de elementos. Al ser un tipo compuesto,
se utiliza cuando la colección de valores del mismo tipo debe almacenarse en una
sola variable. En Rust, un array solo puede tener una longitud fija. Al igual
que en todos los demás lenguajes, a cada elemento del array se le asigna un índice.
Por defecto, el primer elemento siempre está en el índice 0.

# Definir un Array

Para definir un array en `Rust`, tenemos que definir el tipo y el tamaño del array.
Para inicializar un array, los elementos están encerrados entre corchetes [].

```rust
fn definir_un_array() {
   // Sintaxis
   // let nombre_array: [tipo; tamaño] = [elem1, elem2, elem3, elem4];

   // definir un array de 4 elementos
   let arr: [i32; 4] = [1, 2, 3, 4];

   // inicializar un array de 4 elementos con valor 0.
   let arr1 = [0; 4];
}
```

La declaración `arr` en la línea 17 declara un array con los elementos 1, 2, 3, 4.

La declaración del array `arr1` en la línea 23, determina implícitamente el tipo de datos (entero)
a partir del valor 0 y 4 es el tamaño de la matriz. Entonces, esto se convierte en una
matriz que consta de 4 ceros.

# Acceder a un elemento de una matriz

Se puede acceder a cualquier valor de la matriz escribiendo el nombre de la matriz
seguido del número de índice entre corchetes [].

```rust
fn acceso_de_elementos() {
   // definir una matriz de 4 elementos
   let arr:[i32; 4] = [22, 13, 9, 55];

   // imprimir el primer elemento de la matriz
   println!("El primer elemento del array es {}", arr[0]);

   // inicializar una matriz de 4 elementos con valor 2
   let arr1 = [2; 4];

   // imprimir el primer elemento de la matriz
   println!("El primer elemento del array es {}", arr1[0]);
}
```

## ¿Cómo hacer un array mutable?

Al igual que una variable se puede hacer mutable al agregar la palabra clave `mut`
después de `let`, lo mismo ocurre con una matriz.

```rust
fn mutabilidad() {
    // definir una matriz mutable de 4 elementos
    let mut arr:[i32; 4] = [7, 8, 9, 10];

    println!("El valor de la matriz en el índice 1: {}", arr[1]);

    arr[1] = 9;

    println!("El valor de la matriz en el índice 1: {}", arr[1]);
}
```

## Imprimir Arrays

Toda la matriz se puede recorrer utilizando un bucle o el `trait` de depuración.

```rust
fn imprimir_arrays() {
    // definir una matriz de 4 elementos
    let mi_array:[i32; 4] = [1, 2, 3, 4];

    // Usar el `trait` de depuración
    println!("\nImprimir utilizando el `trait` de depuración");
    println!("Mi array: {:?}", mi_array);
}
```

## Obtener la longitud de un array

Para acceder a la longitud de la matriz, debemos utilizar la función incorporada `len`.

```rust
fn longitud_de_un_array() {
    // definir una matriz de 4 elementos
    let mi_array:[i32; 4] = [8, 66, 22, 11];

    // imprimir la longitud de la matriz
    println!("Longitud del array: {}", mi_array.len());
}
```

## Obtener un Slice apartir de un array

Slice es básicamente una porción de una matriz. El tamaño de la porción se conoce
en tiempo de compilación.

## Sintaxis

Para declarar un segmento de un array, debemos especificar el nombre del conjunto fuente
y el rango de elementos que se incluirán en el segmento.

```rust
fn obtener_un_slice() {
    // definir una matriz de 4 elementos
    let mi_array:[i32; 4] = [1, 2, 3, 4];

    // definir el slice
    let slice_desde_un_array: &[i32] = &mi_array[0..2];

    // imprimir el segmento de una matriz
    println!("Slice de una matriz: {:?}", slice_desde_un_array);
}
```
