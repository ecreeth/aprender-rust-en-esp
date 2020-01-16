# Pasar valores por referencia

Cuando deseamos que la función llamada realice cambios en los parámetros
de modo que la función de llamada vea los cambios cuando la llamada regrese.
El mecanismo para hacer esto se llama pasar argumentos por referencia.

📎 Ejemplo:

El siguiente ejemplo crea una función square() que toma un número `num` que se pasa
como referencia a la función como parámetro e imprime el cuadrado de la
función dentro de la función.

```rust, editable
fn square(num:&mut i32){
  *num = (*num * *num);
  println!("El valor de `num` dentro de la función es: {}", num);
} 

fn main() {

  let mut num = 4;
  println!("El valor de `num` antes de la llamada a la función es: {}", num);

  println!("Invocar la función");
  square(&mut num);

  println!("El valor de `num` después de la llamada a la función es: {}", num);
}
```

## Explicación

El programa anterior comprende dos funciones, la función definida por el usuario square()
y la función principal main() donde se llama a la función.

### Función definida por el usuario

La función square() se define de la línea 13 a la línea 16, que toma una referencia mutable (&mut)
al parámetro `num` del tipo `i32`.

    - En la línea 14, se calcula el cuadrado de la variable `num`. Dado que `num` es una referencia
	  a una variable, para acceder al valor de la variable referenciada, se requiere una desreferenciación
	  Eso se logra con el (*n). En el lado derecho, se accede al valor al que hace referencia `num` y se
	  multiplica consigo mismo. La asignación también es a (*n), lo que significa que el resultado calculado
	  se almacena en la variable a la que `num` hace referencia.

    - El cuadrado de la función se imprime en la línea 15.

### Función principal

  - En la línea 20, se define una variable mutable llamada `num`.
  - En la línea 24, se invoca la función square(). El argumento de esta función es (&mut num). Aquí, (&) indica que
    es una referencia a la variable `num` y `mut` indica que `num` se puede cambiar dentro de la función square().
  - Después de la llamada a la función, se imprime el valor de `num`.

-------------------------

> 📝 Nota: El valor de `num` cambia dentro de la función square().

> 📝 Nota: El argumento, así como el parámetro, se establece como una referencia mutable cuando
el valor se pasa por referencia. Si el valor se va a actualizar, primero se desreferencia y
luego se realiza la operación de actualización.