# Introducción a los módulos

Los módulos son una colección de elementos que pueden contener estructuras,
funciones, enumeraciones, vectores, matrices, etc.

💡 ¿Por qué hacer un módulo?

 - El código del programa se organiza.
 - puedes usar el mismo nombre para cosas como una estructura. Por ejemplo, puede usar el
   nombre `Configuration` en diferentes módulos y codificarlo de manera diferente. De lo
   contrario, tendría diferentes nombres torpes para la estructura como
 - EngineConfiguration, ConsoleConfiguration, etc.

## Declarar un módulo

Para declarar un módulo en `Rust`, utilice la palabra clave `mod` seguida del nombre del 
módulo y el cuerpo del módulo entre llaves {}.

⚠️ Convención a la hora de nombrar los módulos

El nombre del módulo debe escribirse en snake_case.

## Invocar un módulo

El módulo se puede invocar desde cualquier lugar dentro del código del programa.

## Palabras clave para módulos

Las siguientes palabras clave se utilizan para declarar módulos:

`mod`: declara un nuevo módulo
`pub`: crea un módulo público
`use`: importa el módulo en el ámbito local

> 📝 Nota: Los módulos se declaran con la palabra clave `mod` y son privados de manera predeterminada.

### Ejemplo

El siguiente ejemplo utiliza la palabra clave mod para declarar un módulo llamado `hola` , y define
una función `imprimir_mensaje` dentro del módulo:

```rust, editable
// Declaración del módulo
mod hola {

  fn imprimir_mensaje(){
    println!("Quetal!, esta es una función del módulo `hola`");
  }

}

// Función principal
fn main() {

  // invocar el módulo 'hola'
   hola::imprimir_mensaje();
}
```

❌: Este código generará un error, porque la función imprimir_mensaje es privada