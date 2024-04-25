pub fn is_armstrong_number(num: u32) -> bool {
    
    //tamaño de digitos en num

    let mut num_copy = num;

    let mut digits = 0;

    while num_copy > 0 {
        num_copy /= 10;
        digits += 1;
    }

    //suma de los digitos elevados a la potencia de la cantidad de digitos

    num_copy = num;


    let mut sum: u64 = 0;

    while num_copy > 0 {
        // digit es el ultimo digito de num_copy                
        let digit = num_copy % 10;
        
        
        sum += digit.pow(digits) as u64;
        
        // se elimina el ultimo digito de num_copy
        num_copy /= 10;
    }

    // compra la suma con el numero original
    // retorna true si son iguales, false si no lo son

    sum == num as u64


    // if sum == num as u64{
    //     return true;
    // } else {
    //     return false;
    // }

}
