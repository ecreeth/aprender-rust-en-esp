# Tipos Numéricos: Enteros y Flotantes

# Enteros

Las variables del tipo de dato `Entero` contienen valores de números enteros.
Hay dos subtipos de tipo de datos enteros en `Rust`, en función del número de
bits ocupados por una variable en la memoria.

## Tipos de tamaño fijo

Los tipos enteros fijos tienen un número específico de bits en su notación.
Esta notación es una combinación de una letra y un número. El primero no es
la categoría del número entero, ya sea sin signo o con signo, y el segundo
no tiene el tamaño de un número entero, es decir, 8, 16, 32, 64.

A continuación se muestra la lista de tipos enteros de longitud fija:

    - i8:  Entero con signo de 8  bits.
    - i16: Entero con signo de 16 bits.
    - i32: Entero con signo de 32 bits.
    - i64: Entero con signo de 64 bits.

    - u8:  Entero sin signo de 8  bits.
    - u16: Entero sin signo de 16 bits.
    - u32: Entero sin signo de 32 bits.
    - u64: Entero sin signo de 64 bits.

## Tipos de tamaño variable

El tipo entero en el que el tamaño particular depende de la arquitectura subyacente de la máquina.

	- isize: Entero con signo del tamaño del puntero.
	- usize: Entero sin signo del tamaño del puntero.

📝 ¿Por qué hay tantos tipos de enteros y cómo se elige un tipo de datos?

La elección depende de qué valores se espera que tenga una variable. Por lo tanto,
un programador debe elegir un tipo de datos que no sea tan pequeño que los datos se pierdan. 
Tampoco deberían elegir un tipo de datos que sea tan grande que desperdicie memoria.

# Ejemplos

## Definición implícita

El siguiente código define explícitamente las variables enteras utilizando
el tipo entero fijo o variable:

```rust
fn definicion_explicita() {

    // definir explícitamente un número entero
    let a:i32 = 24;
    let b:u64 = 23;
    let c:usize = 26;
    let d:isize = 29;

    // imprimir los valores de las variables
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}
```

## Definición explícita

El siguiente código define implícitamente el tipo entero de la variable mediante
la asignación de un valor entero a la variable.

```rust
fn definicion_implicita() {

    // definir explícitamente un número entero
    let a = 21; 
    let b = 1;
    let c = 54;
    let d = 343434;

    // imprimir los valores de las variables
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}
```

# Flotantes

Los números de coma flotante se refieren a números con una parte fraccionaria.
Hay dos subtipos: punto flotante `f32` de precisión simple y punto flotante `f64` de
precisión doble, y este último tiene más bits para almacenar el número.

	- f32: Flotante de 32 bits.
    - f64: Flotante de 64 bits.

Ejemplos

## Definición explícita

El siguiente código define explícitamente la variable flotante utilizando el tipo flotante (`f32` o `f64`):

```rust
fn definicion_explicita() {

    // definir explícitamente un tipo flotante
    let f1:f32 = 32.9;
    let f2:f64 = 6789.89;

    println!("Valor de f1: {}", f1);
    println!("Valor de f2: {}", f2);
}
```

## Definición implícita

El siguiente código define implícitamente el tipo flotante de la variable mediante la asignación
de un valor de punto flotante a la variable:

```rust
fn definicion_implicita() {
    // definir implícitamente un tipo flotante
    let pi = 3.14;
    let e = 2.17828;

    println!("Valor de pi: {}", pi);
    println!("Valor de e: {}", e);
}
``` 