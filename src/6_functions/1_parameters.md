# Funciones con Parámetros

En el ejemplo anterior, se definió una función sin nada dentro de los corchetes.
Pero ciertas funciones requieren cierta información sobre la cual deben operar.
Por ejemplo, una función que se espera que calcule el cuadrado de un número debe
proporcionarse con el número mismo. Eso es lo que es un parámetro.

## ¿Cuáles son los parámetros?

Las variables o valores que van en la definición de la función son parámetros.

## ¿Qué son los argumentos?

Las variables o valores que van en su lugar en la invocación de la función
se conocen como argumentos.

📎 Ejemplo:

Para comprender el concepto anterior, veamos el siguiente ejemplo:

```rust, editable
// creamos la función
fn mostrar_valores(param_1: i32, param_2: i32) {
  println!("El primer valor pasado dentro de la función: {}", param_1);
  println!("El segundo valor pasado dentro de la función: {}", param_2);
}

fn main() {
  let valor_1 = 1;
  let valor_2 = 2;

  // Llamar la función
  mostrar_valores(valor_1, valor_2);
  println!("Fin de la función principal");
}
```

# Tipos de argumentos

Los argumentos se pueden pasar a una función de dos maneras diferentes:

	- Pasar por valor
	- Pasar por referencia