# Devolver un valor de una función

Las funciones pueden devolver un valor utilizando la palabra clave `return`
dentro de la definición de la función. Después de ejecutar la declaración de
devolución, el control vuelve a la persona que llama. Una invocación de función
se reemplaza con el valor que devuelve la llamada. Por lo tanto, ese valor
se puede guardar en una variable.

La sintaxis general para devolver un valor de una función utilizando la palabra clave return:

Simplemente se escribe el valor de retorno, el compilador lo interpretará debido
al signo `->` en la definición de la función.

-------------------------------------

## 📎 Ejemplo #1

El siguiente ejemplo crea una función square() que toma un número `num` como parámetro de la función
y almacena el cuadrado de `num` en la variable local `res` y devuelve la variable `res`.

```rust, editable
fn square(num: i32) -> i32 {

  println!("El valor de `num` dentro de la función es: {}", nun);

  let res = num * num;

  // devuelve el cuadrado del número `num`
  res
}

fn main() {

  let num = 4;

  println!("El valor de `num` fuera de la función es: {}", n);
  println!("Invocar la función");

  println!("\nSalida : {}", square(num));
}
```

## 📎 Ejemplo #2

El siguiente ejemplo crea una función square() que toma un número `num` como parámetro de la función
y devuelve el cuadrado del número `num` mediante la palabra clave `return`.

```rust, editable
fn square(num: i32) -> i32 {

  println!("El valor de `num` dentro de la función es: {}", nun);

  return num * num;
}

fn main() {

  let num = 4;

  println!("El valor de `num` fuera de la función es: {}", n);
  println!("Invocar la función");

  println!("\nSalida : {}", square(num));
}
```