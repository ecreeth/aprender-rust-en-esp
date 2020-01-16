# Pasar parámetros por valor

## Argumentos pasados por valor:

Los valores de la función de llamada se copian a los parámetros de la función
llamada en el momento en que se llama a la función. La función llamada puede
cambiar los valores de las variables de parámetros todo lo que quiera. Este
cambio no se reflejará en las variables pasadas como argumentos en la
función de llamada.

📎 Ejemplo:

El siguiente ejemplo crea una función `square()` que toma un número `num` como
parámetro de la función e imprime el cuadrado.

```rust, editable
fn square(mut num: i32){
  num = num * num;
  println!("El valor de `num` dentro de la función es: {}", num);
}

fn main() {
  let num = 4;

  println!("El valor de `num` antes de la llamada a la función es: {}", num);
  
  println!("Invocamos la función");
  square(num);

  println!("\nEl valor de `num` después de la llamada a la función es: {}", num);
}
```

## Explicación

El programa anterior consta de dos partes, la función definida por el usuario square()
y la función principal main() donde se llama a la función.

### Función definida por el usuario
	- En la línea 17, `num` se multiplica consigo mismo y el valor se guarda en `num`.
	- El cuadrado del argumento así calculado se muestra en la pantalla en la línea 18.

### Función principal

	- En la línea 23, se define una variable `num`.
	- En la línea 28, se invoca la función square(), que toma `num` como argumento de la función.
	- Después de la llamada a la función, se imprime el valor de `num`.

> 📝 Nota: el valor de `num` no cambia