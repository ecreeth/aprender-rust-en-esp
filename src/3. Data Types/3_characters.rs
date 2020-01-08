//! Caracteres

//! El valor asignado a una variable `char` está encerrado en una comilla simple ('').

//! 📝: A diferencia de otros lenguajes, un caracter en `Rust` ocupa 4 bytes en lugar de un solo byte.
//! Esto es debido a que se pueden almacenar mucho más que solo un valor `ASCII` como emojis, caracteres
//! coreanos, chinos y japoneses.


//! Ejemplos

//! ## Definición explícita

//! El siguiente código define explícitamente la variable usando la palabra clave char:

fn definicion_explicita() {

    // definir explícitamente 
    let letra:char = 'l';

    println!("Valor del caracter: {}", letra); 
}

//! ## Definición implícita

//! El siguiente código define implícitamente el tipo de caracteres de la variable al asignarles 
//! el valor único entre comillas simples.

fn definicion_implicita() { 

    // definir implícitamente
    let letra_1 = 'a';
    let letra_2 = 'b';

    println!("Valor de la letra_1: {}", letra_1);
    println!("Valor de la letra_2: {}", letra_2);
}