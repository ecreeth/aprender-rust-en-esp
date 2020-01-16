# Introducción a los Operadores

Un operador es un símbolo que toma uno o más valores y genera otro. Le dice al 
compilador que realice algún tipo de operación.

## Tipos de operadores

Hay diferentes operadores disponibles en `Rust` para realizar diferentes operaciones.
Según el número de operandos, los operadores se pueden clasificar en operadores 
binarios y unarios:

### Operadores Unarios

Los operadores que actúan sobre un solo operando son operadores unarios.

#### Tipos

    - Expresión prestada (Borrow Expression) -> (&, &mut)
    - Expresión de desreferencia (Dereference Expression) -> (*)
    - Expresión de negación -> (!)
    - Expresión de negación lógica

### Operadores binarios 

Los operadores que tratan con dos operandos son operadores binarios.

#### Tipos

    - Expresión Aritmética -> (+, -, *, /, %)
    - Expresión lógica -> (&&, ||)
    - Expresión de comparación -> (>, <, <=, >=, ==, !=)
    - Expresión de asignación -> (=)
    - Expresión de asignación compuesta -> (-=, +=, /=, %=, *=)
    - Expresiones bit a bit -> (&, |, ^)
    - Expresión tipográfica -> (as)
