//! Rust es un lenguaje de tipo estático, lo que significa que se debe conocer
//! el tipo de todas las variables en tiempo de compilación.

//! ¿Cómo se establecen los tipos en `Rust`

//! Podemos definir una variable en `Rust` de dos maneras diferentes:

// ## Definición implícita

//! A diferencia de otros lenguajes como `C++` y `Java`, `Rust` puede inferir
//! el tipo de dato del tipo de valor asignado a una variable.

fn definicion_implicita() {
	let nombre_variable = "valor";
}

//! ## Definición explícita

// Explica explícitamente al compilador sobre el tipo de variable.

fn definicion_explicita() {
	// let nombre_variable: tipo_de_dato = valor;
	let nombre_variable: u32 = 23;
}

//! # Tipos primitivos

//! Rust tiene un par de tipos que se consideran primitivos. Eso significa que están
//! integrados en el idioma. Hay diferentes tipos de datos utilizados para diferentes propósitos.

//! La siguiente ilustración muestra los diferentes tipos de datos primitivos en Rust:

/*

								[Tipos de Datos Primitivos]
									|				   |
								[Escalar]			[Compuestos]
								  |					   |	 |
		[Numerico]		 	[No-Numerico]			[Array][Tuplas]
		   |  |		           |	  |
	 [Intero][Flotante]	 [Boleano] [Caracter]
*/