//! # Diferencias entre variables y constantes

//! No poder cambiar el valor de una variable podría haberte recordado a las 
//! constantes en otros lenguajes. Al igual que las variables inmutables, las 
//! constantes son valores vinculados a un nombre y no se les permite cambiar, 
//! pero existen algunas diferencias entre constantes y variables.

//! La palabra clave `mut` no es utilizada con constantes y estas siempre son inmutables.

//! Las constantes las declaramos utilizando la palabra clave `const` en lugar de 
//! la palabra clave `let`, y el tipo del valor debe ser asignado (que debemos asignarle 
//! un tipo de dato al momento de declararlas). No te preocupes por estos conceptos de 
//! contantes, ya que lo veremos en el próximo capítulo. Lo único importante es que sepas 
//! que siempre se debe asignar el tipo en la declararación de las constantes.

//! Las constantes se pueden declarar en cualquier ámbito, incluido el ámbito global, lo que las 
//! hace útiles para valores que muchas partes del código necesitan conocer.

//! Aquí hay un ejemplo de la declaración de una constante, donde el nombre de la misma es 
//! PUNTOS_MAXIMOS y ​​su valor se establece en 100,000. (La convención de nomenclatura de `Rust` para 
//! las constantes es usar todas las mayúsculas con guiones bajos entre palabras, y los guiones 
//! bajos se pueden insertar en literales numéricos para mejorar la legibilidad): 

fn main() {
	// Declaramos nuestra constante, le asignamos el tipo de dato y su valor
	const PUNTOS_MAXIMOS: u32 = 100_000;
}

//! Las constantes son válidas durante todo el tiempo dentro del alcance en el que han 
//! sido declaradas, lo que las convierte en una opción útil para los valores en el dominio 
//! de su aplicación que varias partes del programa pueden necesitar conocer.

//! Nombrar valores codificados utilizados en todo el programa como constantes es útil para 
//! transmitir el significado de ese valor a los futuros mantenedores del código. También ayuda 
//! tener un solo lugar en su código que necesitaría cambiar si el valor codificado necesita ser 
//! actualizado en el futuro.