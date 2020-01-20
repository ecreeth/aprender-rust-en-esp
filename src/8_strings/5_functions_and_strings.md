# Funciones y Strings

## Pasar cadena primitiva: cadena literal (&str)

Los literales de cadena se pasan a las funciones al igual que otras variables. Se pueden reutilizar
después de la llamada a la función.

```rust
fn main(){
   let course: &str = "Rust Programming";
   mostrar_nombre_curso(course); 
   println!("{}",course); // string literal is used after the function call
}
fn mostrar_nombre_curso(mi_curso: &str){
   println!("Curso : {}", mi_curso);
}
```

## Pasando String Growable - Objeto String

Al pasar objetos de cadena a funciones, no se pueden reutilizar nuevamente porque el valor una vez pasado
se mueve al alcance de esa función y no se puede reutilizar.

```rust
fn main(){
   let course:String = String::from("Rust Programming");
   mostrar_nombre_curso(course); 
   //cannot access course after display
}
fn mostrar_nombre_curso(mi_curso: String){
   println!("Cruso : {}", mi_curso);
}
```