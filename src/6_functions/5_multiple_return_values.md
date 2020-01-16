# Función con múltiples valores de retorno

En lenguajes de programación del sistema como C++ y C, solo es posible devolver
un valor único o un puntero a una matriz desde una función. Sin embargo, `Rust`
le permite devolver múltiples valores utilizando una tupla.

## 📎 Ejemplo:

El siguiente ejemplo hace una función cal_perimetro_area() que toma una `x` e `y`
(longitud y ancho de un rectángulo) como parámetro de la función y devuelve una
tupla (área, perímetro).

```rust, editable
// función principal
fn main() {
    let longitud = 4;
    let ancho = 3;

    println!("Longitud del rectángulo: {}", longitud);
    println!("Ancho del rectángulo: {}", ancho);

    let (area, perimetro) = cal_perimetro_area(longitud, ancho);

    println!("Área: {}, perímetro: {}", area, perimetro);
}

// calcular el área y el perímetro
fn cal_perimetro_area(x: i32, y: i32) -> (i32, i32) {

    // calcular el área y el perímetro del rectángulo
    let area = x * y;
    let perimetro = 2 * (x + y);

    // devolver el área y el perímetro del rectángulo
    (area, perimetro)
}
```

## Explicación

El programa anterior comprende dos funciones, la función definida por el usuario cal_perimetro_area()
y la función de controlador main() donde se llama a la función.

### Función definida por el usuario

La función cal_perimetro_area() se define de la línea 28 a la línea 36.

    - En la línea 31, el área del rectángulo se calcula multiplicando los parámetros `x` e `y`
      y el resultado se guarda en la variable `area`.
    - En la línea 32, el perímetro del rectángulo se calcula sumando los parámetros `x` e `y`
      luego multiplicando el resultado por 2 y luego, el resultado final se guarda en la
      variable `perimetro`.
    - En la línea 35, se devuelve una tupla (area, perimetro).

### Función principal

    - En la línea 3, se inicializa una longitud variable con el valor 4.
    - En la línea 4, se inicializa un ancho variable con el valor 3.
    - En las líneas 5 y 6, se muestra el valor de longitud y ancho, respectivamente.
    - En la línea 7, se invoca la función cal_perimetro_area() que toma largo y ancho
      como argumento para la función y el valor de retorno de la función se guarda en una tupla.