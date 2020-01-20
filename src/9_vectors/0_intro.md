# Introducción a los Vectores

Los Vectores son conjuntos de tamaños redimensionables (pueden crecer o reducir su tamaño).

## Crear vectores

Hay dos formas de crear un vector:

### Sintaxis

Para crear un vector, escriba la macro vectorial (vec!) Seguida de los elementos del vector encerrados entre corchetes

Es opcional definir el tipo y el tamaño del vector encerrado entre corchetes angulares. Use la macro vector (vec!)
Antes de definir los elementos del vector.

```rust
fn main() {
   //define a vector of size 4
   let my_vec = vec![1, 2, 3, 4, 5];
   //print the vector
   println!("{:?}", my_vec);
}
```

> Nota: Similar a los `Arrays`, estos se pueden mostrar en la pantalla usando la macro `println!()`.


# Acceder a un elemento de un vector

Se puede acceder a cualquier valor del vector escribiendo el nombre del vector seguido del número de índice entre corchetes `[]`.

```rust
fn main() {
   //define a vector of size 4
   let my_vec = vec![1, 2, 3, 4, 5];
   //access a particular value
   println!("{}", my_vec[0]);
}
```

> Nota: Si intenta acceder a un índice que no existe, el compilador emitirá un error de acceso limitado, ❌.

Esto se ilustra en el siguiente código:

```rust
fn main() {
   //define a vector of size 4
   let my_vec = vec![1, 2, 3, 4, 5];
   //access a particular value
   eprintln!("{}", my_vec[9]);
}
```

Para atender las excepciones limitadas, puede usar una palabra clave `None`.

```rust
fn main() {
    let my_vec = vec![1, 2, 3,4,5];
    match my_vec.get(9) {
        Some(x) => println!("Value at given index:{}", x),
        None => println!("Sorry, you are accessing a value out of bound")
    }
}
```

# Imprimir el Vector

Todo el vector puede imprimirse utilizando un bucle o el `trait` de depuración.

```rust
fn main() {   
    println!("Print using debug trait");   
    let my_vec = vec![1, 2, 3,4,5];
    //using debug trait 
    println!("Vector : {:?}", my_vec);
    println!("Print using for loop"); 
    // using loop
    let mut index = 0;
    for i in my_vec {
        println!("Element at index {}:{} ", index, i);
        index = index+1;
    }
}
```

# Métodos de vectores

Los métodos de los vectores se resumen en el cuadro a continuación:

```rust
fn main() {

   let mut my_vec = Vec::new();

   println!("Empty Vector : {:?}", my_vec);
   my_vec.push(1);
   my_vec.push(2);
   my_vec.push(3);
   println!("Pushed elements 1 , 2 , 3 : {:?}", my_vec);
   my_vec.pop();
   println!("Popped value: {}", 3);
   println!("Popped element at last index : {:?}", my_vec);
   my_vec.remove(1);
   println!("Removed value: {}", 2);
   println!("Removed element at index 1 : {:?}", my_vec);
   println!("Size of vector is :{}", my_vec.len());
   println!("Does my vector contains 1 : {}", my_vec.contains(&1));
}
```

> Nota: Cuando use la función `.contains()`, considere borrowing(pedir prestado) el valor. La razón será más clara una vez que
discutamos diferentes tipos de operaciones de préstamo en el capítulo posterior.