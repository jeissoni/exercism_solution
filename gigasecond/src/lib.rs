use time::PrimitiveDateTime as DateTime;

// Función para calcular un gigasegundo después de una fecha y hora dadas
pub fn after(start_date: DateTime) -> DateTime {
    const GIGASECOND: i64 = 1_000_000_000;
    // Añadir un gigasegundo a la fecha y hora de inicio
    start_date + time::Duration::seconds(GIGASECOND)
}