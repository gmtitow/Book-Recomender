extern crate serde_json;
use std::collections::HashMap;

pub fn get_words(text: &str)->Vec<&str>{
    text.split(|c: char| !c.is_alphanumeric()).collect()
}

pub fn count_terms(terms: Vec<&str>)->HashMap<&str,i32> {
    let mut map = HashMap::new();

    for term in terms {
        match map.get_mut(&term) {
            Some(value)=> *value = *value+1,
            None => {map.insert(term,1);},
        }
    }

    map.remove("");

    return map;
}

pub fn convert_to_json(map: &HashMap<&str, i32>)->String{
    serde_json::to_string(map).expect("Не удалось сериализовать")
}