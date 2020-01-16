# Booleanos

Las variables booleanas puede tomar un valor verdadero o falso. 

## Ejemplos

El siguiente código explica cómo definir una variable booleana de *TRES* maneras diferentes:


## Definición explícita

El siguiente código define explícitamente la variable usando la palabra clave `bool`:

```rust, editable
fn definicion_explicita() {
	// asignar un valor booleano
    let esta_disponible:bool = true;

    println!("Valor: {}", esta_disponible);
}
```

## Definición implícita

El siguiente código define implícitamente el tipo booleano de una variable asignando 
el valor verdadero o falso a la variable.

```rust, editable
fn definicion_implicita() {
    // asignar un valor booleano
    let a = true;
    let b = false;

    println!("valor de a: {}", a);
    println!("valor de b: {}", b);
}
```

## Como resultado de una expresión

El resultado de una expresión que se evalúa como verdadero o falso (por ejemplo, una comparación
de dos valores) se puede asignar a una variable booleana implícita.

```rust, editable
fn resultado_de_una_expresion() {
    // obtener un valor de una expresión
    let c = 10 > 2;

    println!("Valor de c: {}", c);
}
```