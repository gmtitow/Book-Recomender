use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

extern crate logger;

use logger::Logger;

pub fn union_to_string(vec : &Vec<String>, separator: &str, addition: Option<&str>) -> String {
    let mut first = true;
    let mut data = String::new();
    for value in vec {
        if first {
            first = false;
        } else {
            data.push_str(separator);
        }
        match addition {
            Some(var) => {
                data.push_str(var);
            },
            None => {}
        };
        data.push_str(&value);
    };
    data
}

pub fn get_words(text: &str)->Vec<&str>{
    text.split(|c: char| !c.is_alphanumeric()).collect()
}

pub fn get_terms(text: &str, term_count: usize)->Vec<String>{

    if term_count == 0 {
        panic!("term count must be 1 or more");
    }

    let mut result: Vec<String> = Vec::new();
    let mut last_result = String::new();

    let mut logger = Logger::new("log_terms.txt");

    let mut index = 0;

    for sym in text.chars() {

        if !sym.is_alphanumeric() {
            continue;
        }

        last_result.push(sym);

        if index < 1000 {
            logger.writeln(&last_result);
        }

        if last_result.chars().count() == term_count {
            result.push(last_result);
            last_result = String::new();
        }

        index = index + 1;
    }

    result
}

pub fn count_terms(terms: Vec<&str>)->HashMap<&str,f32> {
    let mut map: HashMap<&str,f32> = HashMap::new();

    for term in terms {
        match map.get_mut(&term) {
            Some(value)=> *value = *value+1.0,
            None => {map.insert(term,1.0);},
        }
    }

    map.remove("");

    return map;
}

pub fn count_terms_string(terms: Vec<String>)->HashMap<String,f32> {
    let mut map: HashMap<String,f32> = HashMap::new();

    for term in terms {
        match map.get_mut(&term) {
            Some(value)=> *value = *value+1.0,
            None => {
                map.insert(term,1.0);
            },
        }
    }

    map.remove("");

    return map;
}

pub fn convert_to_json<T: serde::ser::Serialize>(map: &HashMap<&str, T>)->String{
    serde_json::to_string(map).expect("Не удалось сериализовать")
}

pub fn convert_to_json_string<T: serde::ser::Serialize>(map: &HashMap<String, T>)->String{
    serde_json::to_string(map).expect("Не удалось сериализовать")
}

pub fn convert_into_string(map: &HashMap<String, f32>)->String{
    let mut result = String::from("{");

    for (key,value) in map.into_iter() {
        result.push_str(&key);
        result.push_str(" : ");
        result.push_str(&value.to_string());
        result.push_str(", ");
    }
    result.push_str("}");

    result
}

pub fn print_in_file(message: &str, file : &mut File){
    file.write(message.as_bytes()).unwrap();
}

pub fn get_vector_len_string(vector: &HashMap<String, f32>)->f64 {
    let mut sum: f64 = 0.0;
    for (key,value) in vector {
        let val: f64 = (*value).into();
        sum = sum + val*val;
    }

    sum.sqrt()
}

pub fn get_vector_len(vector: &HashMap<&str, f32>)->f64 {
    let mut sum: f64 = 0.0;
    for (key,value) in vector {
        let val: f64 = (*value).into();
        sum = sum + val*val;
    }

    sum.sqrt()
}

pub fn output_hash_map(vector: &HashMap<String, f32>, logger: &mut Logger){
    for (key,value) in vector {
        logger.writeln(&format!("{0} = {1}",key,&value.to_string()));
    }
}

pub fn divide_word(word: &str, dividers: &Vec<&str>, ignore: Option<Vec<&str>>) -> Vec<String> {
    let mut result = Vec::new();

    match &ignore {
        Some(ignore_list) => result = divide_word(word, &ignore_list, None),
        None => {
            result.push(word.to_owned());
        }
    };

    let mut last_divider: bool;

    for divider in dividers {
        let mut all_parts: Vec<String> = Vec::new();
        for word in &result {
            if let Some(ignore_list) = &ignore {
                if ignore_list.contains(&word.as_str()) {
                    all_parts.push(word.to_string());
                    continue;
                }
            }

            if word == "" {
                continue;
            }

            last_divider = false;
            if dividers.contains(&word.as_str()) {
                all_parts.push(word.to_owned());
            } else {
                let parts: Vec<&str> = word.split(divider).collect();

                if parts.len() > 1 {
                    for part in parts {
                        if part.trim() != "" {
                            all_parts.push(part.to_owned());
                        }
                        all_parts.push(divider.to_string());

                        last_divider = true;
                    }
                } else {
                    all_parts.push(word.to_owned());
                }
            }

            if last_divider {
                all_parts.pop();
            }
        }

        result.clear();

        for part in all_parts {
            result.push(part);
        }
    }

    match &ignore {
        Some(ignore_list) => {
            let mut new_result: Vec<String> = Vec::new();

            let mut next_concate = false;

            for res in result {
                if ignore_list.contains(&res.as_str()) {
                    let mut complete: String;

                    if new_result.len() > 0 {
                        complete = new_result.pop().unwrap();
                    } else {
                        complete = String::new();
                    }
                    complete.push_str(&res);

                    new_result.push(complete);

                    next_concate = true;
                } else {
                    if next_concate && new_result.len() > 0{
                        let mut complete = new_result.pop().unwrap();
                        complete.push_str(&res);
                        new_result.push(complete);
                        next_concate = false;
                    } else {
                        new_result.push(res);
                    }
                }
            }

            result = new_result;
        }
        None => {}
    };

    result
}

pub fn handle_word(word: &str) -> String {
    let mut handled_word: String = String::new();

    let mut saved = String::new();
    for ch in word.chars() {
        if !ch.is_alphanumeric() {
            if handled_word == "" {
                continue;
            } else {
                saved.push(ch);
            }
        } else {
            if saved == "" {
                handled_word.push(ch);
            } else {
                handled_word.push_str(saved.as_str());
                handled_word.push(ch);
                saved = String::new();
            }
        }
    }

    handled_word
}