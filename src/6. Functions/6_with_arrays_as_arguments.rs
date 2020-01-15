//! Funciones con arrays como argumentos

//! A menudo es necesario pasar matrices como argumentos a las funciones.
//! Rust permite al programador pasar matrices ya sea por valor o por referencia.

//! ------------------------------------- 

//! # Pasar por valor

//! Las matrices se pueden pasar a una función por valor. Lo que eso significa es que se
//! hace una copia de la matriz de la función de llamada a la función llamada.

//! La sintaxis general para pasar una matriz por valor a una función es:

//! fn nombre_funcion( mut nombre_array: [tipodato;tamanho] )

//! 📎 Ejemplo #1:

//! El siguiente ejemplo toma la matriz arr por valor en el parámetro de función.

fn main() {

   let arr = [1, 2, 3, 4, 5];

   modificar_mi_array(arr);

   println!("Array en la función principal: {:?}", arr);
}

fn modificar_mi_array(mut arr:[i32;5]) {
   arr[2] = 8;
   arr[3] = 9;
   println!("Array en la función `modificar_mi_array`: {:?}", arr);
}

//! 📝 Nota: La palabra clave mut al pasar una matriz por valor es opcional. Se escribe junto con
//! el nombre de la matriz si se desea realizar cambios locales. Se puede omitir de otra manera.

//! Ejemplo #2

//! El siguiente ejemplo realiza una función calcular_medio() que calcula la media de los valores en una
//! matriz tomando primero una suma dentro de un bucle for y luego dividiendo el resultado por 5.

fn main() {

   let arr = [1, 2, 3, 4, 5];

   println!("Array en la función principal: {:?}", arr);

   calcular_medio(arr);
}

fn calcular_medio(arr:[i32;5]){

   let mut sum = 0;

   for i in 0..5 {
       sum += arr[i];
   }

   println!("Media de los valores del array: {}", sum / 5);
}

//! ------------------------------------- 

//! # Pasar por referencia

//! Los arrays se pueden pasar por referencia en el parámetro de función. En otras palabras, los cambios
//! se realizan en la matriz original y no se realiza ninguna copia cuando se pasa por referencia en la función.

//! La sintaxis general para pasar una matriz por referencia a una función es:

//! fn nombre_funcion(nombre_array:&mut [tipodato;tamanho]) 

//! 📎 Ejemplo:

//! El siguiente ejemplo toma la matriz `arr` por referencia en el parámetro de función.

fn main() {

   let mut arr = [1, 2, 3, 4, 5];

   modificar_mi_array(&mut arr);

   println!("Array en la función principal: {:?}", arr);
}

fn modificar_mi_array(arr:&mut [i32;5]){
   arr[2] = 8;
   arr[3] = 9;
   println!("Array en la función `modificar_mi_array`: {:?}", arr);
}

//! # Devolver una matriz

//! Las matrices se pueden devolver desde la función y la sintaxis general para devolver
//! una matriz de una función es:

//! fn nombre_funcion()->[tipodato;tamanho]

//! 📝 Nota: Aquí también se pueden pasar los parámetros.

//! 📎 Ejemplo:

//! El siguiente ejemplo toma el arreglo `arr`, lo modifica dentro de la función y lo devuelve.

fn main() {

   let arr = [1, 2, 3, 4, 5];

   modificar_mi_array(arr);

   println!("Array en la función principal: {:?}", arr);
   println!("Array después de llamar la función: {:?}", modificar_mi_array(arr));
}

fn modificar_mi_array(mut arr:[i32;5])->[i32;5]{
   arr[2] = 8;
   arr[3] = 9;
   arr
}