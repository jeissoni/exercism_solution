use std::collections::HashSet;



pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    //se crea un HashSet vacio
    let mut anagrams: HashSet<&'a str> = HashSet::new();


    //se convierte la palabra a un arreglo de caracteres
    let mut word_array = word.to_lowercase().chars().collect::<Vec<_>>();

    //se ordena el arreglo de caracteres
    word_array.sort();

    //se itera sobre el arreglo de palabras
    for possible_anagram_item in possible_anagrams {
        //compara si la palabra es igual a la posible anagrama
        //si es igual no se agrega al HashSet
        if word.to_lowercase() == possible_anagram_item.to_lowercase() {
            continue;
        }

        //se convierte la palabra a un arreglo de caracteres
        let mut possible_word_array = possible_anagram_item.to_lowercase().chars().collect::<Vec<_>>();

        //se ordena el arreglo de caracteres
        possible_word_array.sort();

        //se compara si la palabra es igual a la posible anagrama
        if word_array == possible_word_array {
            //se agrega la palabra al HashSet
            anagrams.insert(*possible_anagram_item);
        }
    }

    //se retorna el HashSet
    anagrams

}