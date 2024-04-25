
/*
Esta línea de código es una declaración de importación en Rust. 
Está trayendo el módulo `fmt` de la biblioteca estándar de Rust, que se denota por `std`.

El módulo `fmt` en Rust contiene funcionalidades para formatear e imprimir. 
Esto incluye rasgos como `Display` y `Debug`, que se pueden implementar para tipos personalizados para controlar cómo se formatean e imprimen. 
También incluye macros como `format!`, `print!` y `println!` para crear e imprimir cadenas formateadas.

Al escribir `use std::fmt;`, estás haciendo que todos los elementos en el módulo `fmt` estén disponibles en el ámbito actual. 
Esto significa que puedes referirte a ellos directamente por sus nombres, sin tener que prefijarlos con `std::fmt::`. 
Por ejemplo, puedes escribir `fmt::Result` en lugar de `std::fmt::Result`.

Esta importación se utiliza típicamente cuando quieres implementar el rasgo `Display` o `Debug` 
para un tipo personalizado, o cuando quieres usar las macros de formateo e impresión.
 */
use std::fmt;




/*
Esta línea de código en Rust es un atributo que se aplica al elemento que le sigue. 
En este caso, es el atributo `derive`, que implementa automáticamente ciertos rasgos para un tipo.

El atributo `derive` está seguido por una lista de rasgos entre paréntesis.
Aquí, `Debug` y `PartialEq` son los rasgos que se están derivando.

El rasgo `Debug` permite que un tipo se formatee para la salida utilizando el especificador de formato 
`{:?}` o `{:#?}`, lo cual es útil para la depuración. Cuando derivas `Debug`, el compilador generará automáticamente 
una implementación de `Debug` que muestra el nombre del tipo y sus campos.

El rasgo `PartialEq` permite la comparación de dos instancias de un tipo para la igualdad (usando `==`) 
y la desigualdad (usando `!=`). Cuando derivas `PartialEq`, el compilador generará automáticamente una 
implementación de `PartialEq` que compara los campos de las instancias.

Al usar `#[derive(Debug, PartialEq)]`, le estás diciendo a Rust que genere automáticamente las implementaciones 
de `Debug` y `PartialEq` para el tipo que sigue a este atributo. Esto puede ahorrarte tener que implementar 
manualmente estos rasgos, lo cual puede ser especialmente útil para tipos complejos.
 */
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

// Implementación de métodos para la estructura Clock
impl Clock {
    // Método new para crear una nueva instancia de Clock
    // El método toma dos argumentos, hours y minutes, y devuelve una instancia de Clock
    // El método se encarga de normalizar las horas y los minutos para que estén dentro del rango válido
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            // Normaliza las horas y los minutos para que estén dentro del rango válido
            // Utiliza el método rem_euclid para asegurarse de que las horas y los minutos sean no negativos
            // Utiliza el método div_euclid para dividir los minutos por 60 y obtener las horas adicionales
            // Utiliza el método rem_euclid para obtener los minutos restantes después de la división
            // Esto asegura que las horas y los minutos estén dentro del rango válido
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    // Método add_minutes para agregar minutos a una instancia de Clock
    // El método toma un argumento, minutes, y devuelve una nueva instancia de Clock
    // El método crea una nueva instancia de Clock con los minutos adicionales
    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Crea una nueva instancia de Clock con las horas y los minutos actuales más los minutos adicionales
        // Utiliza el método new para crear la nueva instancia de Clock
        Clock::new(self.hours, self.minutes + minutes)
    }
}

// Implementación de la función Display para la estructura Clock
// Esto permite que una instancia de Clock se formatee como una cadena cuando se imprime
// La implementación de Display define cómo se formatea una instancia de Clock cuando se convierte en una cadena
impl fmt::Display for Clock {
    // Método fmt para formatear una instancia de Clock como una cadena
    // El método toma una referencia mutable a self y un formateador, y devuelve un resultado
    // El método formatea las horas y los minutos de la instancia de Clock en el formato "HH:MM"
    // Utiliza el método write! para escribir las horas y los minutos en el formateador
    // Esto permite que la instancia de Clock se convierta en una cadena con el formato deseado
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
