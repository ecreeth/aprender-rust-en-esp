# Recursividad

La recursión es un método de invocación de funciones en el que una función se
llama a sí misma durante la ejecución.

Hay problemas que se definen naturalmente de forma recursiva. Por ejemplo, el factorial
de un número nnn se define como n veces el factorial de (n − 1).

## Partes de la recursividad

En términos de programación, una función recursiva debe comprender dos partes:

Caso Base

    - Una función recursiva debe contener un caso base. Esta es una condición para la terminación de la ejecución.

Caso Recursivo
                
    - La función sigue llamándose una y otra vez hasta que se alcanza el caso base.

### 📎 Ejemplo:

El siguiente ejemplo calcula el factorial de un número usando recursividad:

📝 Nota: Un factorial se define solo para números enteros no negativos.

```rust
fn main(){
    let n = 4;
    let fact = factorial(n);

    // imprimir el factorial
    println!("factorial({}): {}", n, fact);
}

// definir la función
fn factorial(n: i64) -> i64 {
    if n == 0 { // caso base
        1
    }
    else {
        n * factorial(n-1) // caso recursivo
    }
}
```

# Explicación

## Función principal

     - En la línea 32, se realiza una llamada a la función factorial con un argumento pasado a la función
       y el valor de retorno se guarda en la variable `fact`.
     - En la línea 35, se imprime el valor de la variable `fact`, es decir, el factorial del número que
       se pasa como argumento.

## Función factorial

 definición de función
     La función toma un parámetro `n` del tipo `i64`.

 cuerpo de la función
 	 La función recursiva se compone de dos partes.

      caso base

          - En la línea 40, se define el caso base. Dado que el valor de `n` disminuye en cada llamada de función
          recursiva, la función termina cuando el valor de `n` es igual a `0` en llamadas recursivas sucesivas.
	
	 caso recursivo

    	 En la línea 44, se define el caso recursivo. El valor n se multiplica por factorial(n-1) y se transfiere a
    	 la pila de memoria. Dado que el valor de `n` disminuye en cada llamada de función, la función sigue
    	 llamándose a sí misma repetidamente hasta que se alcanza el caso base. Tan pronto como se alcanza el
    	 caso base, se calcula el factorial(0) y el valor se utiliza en la expresión
    	 inmediata en la pila de memoria.