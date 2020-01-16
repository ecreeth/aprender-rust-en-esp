Control de visibilidad dentro del mismo archivo usando 'pub'

La palabra clave `pub` hace que el elemento sea público y visible fuera de su alcance.

# Reglas de privacidad

Las siguientes son dos reglas de privacidad al declarar módulos:

📝 Regla No: 1

Si un elemento es público, se puede acceder desde cualquier lugar, es decir, desde la
función principal o desde cualquier otro módulo.

📎 Ejemplo: invocar una función pública directamente #

El siguiente ejemplo declara una función public imprimir_mensaje() dentro del módulo `hello`:

```rust
Declaración del módulo
mod hola {

  pub fn print_statement(){
    println!("Quetal!, esta es una función del módulo `hola`");
  }

}

Función principal
fn main() {

  println!("Hola Mundo! desde la función principal");

  invocar el módulo 'hola'
   hola::imprimir_mensaje();
}
```

📝 Regla No: 2

Si un elemento es privado, se puede acceder a él utilizando su módulo principal, lo que
significa que se puede acceder dentro del módulo pero no fuera de él.

📎 Ejemplo: invocar una función privada indirectamente a través de una función pública

El ejemplo declara un módulo mod `hola` que tiene dos funciones:

	- Una función pública mi_funcion_publica()
	- Una función privada mi_funcion_privada()

💡: `self` puede referirse a una función o cualquier elemento dentro del mismo módulo.

```rust
declare a module
mod hola {

  fn mi_funcion_privada(){
    println!("Hola, soy una función privada dentro del módulo.");
  }

  pub fn mi_funcion_publica(){

    println!("Hola, soy una función pública dentro del módulo.");
    println!("Invocaré una función privada dentro del módulo");

    también funciona sin escribir `self` i.e.
    mi_funcion_privada();
    self::mi_funcion_privada();
  }
}

función principal
fn main() {

  println!("Hola, desde la función principal");

  invocar la función desde el módulo `hola` 
   hola::mi_funcion_publica();
}
```

💡: Si un elemento es privado, se puede llamar desde el módulo secundario.

📎 Ejemplo: acceder a una función privada a través de un módulo secundario

	- 📝: Si hay un módulo dentro del módulo, el módulo externo se llama módulo primario
	- y el módulo dentro del módulo primario se llama módulo secundario. Esto se conoce
	- como un módulo anidado. Esto se tratará con más detalle en próximas lecciones.

El ejemplo declara un `mod` de módulo `modulo_externo` que tiene:
	
	- Una función privada mi_funcion_privada().
	- `modulo_interno`:
			- El módulo interno tiene una función pública mi_funcion_publica()

El siguiente ejemplo muestra cómo se accede a la función privada en el módulo secundario
utilizando la palabra clave super seguida de `::` y el nombre de la función en el módulo primario.

La palabra clave `super` se refiere al módulo padre.

```rust
// función principal
fn main() {
  println!("Hola, desde la función principal");
  modulo_externo::modulo_interno::mi_funcion_publica();
}

// declarar el módulo
mod modulo_externo {

  // funcion dentro del módulo externo
  fn mi_funcion_privada() {
    println!("Hola, entré en la función privada del módulo externo");
  }

  // declarar un módulo anidado
  pub mod modulo_interno {

    // funcion dentro del módulo anidado
    pub fn mi_funcion_publica() {
      println!("Hola, entré en la función pública del módulo interno");
      println!("Invocaré la función privada del módulo externo");
      super::mi_funcion_privada();
    }

  } // fin del `modulo_interno`
} // fin del `modulo_externo`
```

Aunque la función mi_funcion_privada() se declara privada, la función main() puede invocarla
indirectamente porque la función que llama es pública.

📎 Ejemplo: acceder a una función raíz

El siguiente ejemplo muestra cómo se puede acceder a la función raíz (una función que existe fuera
del módulo) dentro de la función de un módulo. Escriba `super::` seguido del nombre de la función raíz.

💡: `super` puede permitir el acceso a una función raíz desde el módulo.

```rust
// función principal
fn main() {
  println!("Hola, desde la función principal");
  mi_modulo ::mi_funcion_publica();
}

fn mi_funcion(){
  println!("Hola, entraste a la función raíz usando super");
}

// declarar un módulo
mod mi_modulo {

  // funcion dentro del módulo externo
  pub fn mi_funcion_publica() {
    println!("Invocar función raíz");
    super::mi_funcion();
  }

} // fin de `mi_modulo`
```

Ahora que hemos aprendido cómo controlar la visibilidad dentro del mismo archivo en un directorio,
aprendamos cómo hacerlo con diferentes archivos.