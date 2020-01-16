# Módulos Anidados

A veces, se requiere la partición lógica de los módulos. Por ejemplo,
hay un libro con un capítulo x, una lección y y un título z. ¿Cómo se
puede realizar la partición lógica? ¡Aquí es donde entra en juego
los módulos anidados!

❓: ¿Qué son los módulos anidados?

Un módulo anidado es un módulo dentro de un módulo.

## Declarar un módulo anidado

Para declarar un módulo anidado, use un bloque `mod` dentro del bloque `mod`.

## Acceda a un módulo anidado

Use el operador de resolución de alcance (::) para acceder al módulo anidado.

```rust
pub mod capitulo {
	// mod nivel 1
  pub mod leccion {
  	// mod nivel 2
    pub mod titulo {
    	// mod nivel 3
        pub fn ilustracion() {
          println!("Hola, soy una función anidada de tercer nivel");
        }
    }
  }
}
fn main() {
   capitulo::leccion::titulo::ilustracion(); // llamar a la función
}
```

¿Qué sucede si tiene muchos niveles de anidamiento en el módulo y desea acceder a una función
en el módulo anidado del quinto nivel, y desea calificar un nombre para los módulos principales
para que pueda acceder fácilmente a una función del módulo en el quinto nivel?

Para comprender este concepto, aprendamos sobre el uso de palabra clave `use` en la próxima lección.