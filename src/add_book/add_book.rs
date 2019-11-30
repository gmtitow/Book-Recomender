use crate::database::common::ColEnum::{Nothing, Null, SomeValue};
use crate::database::common::{ColEnum, IntCol, StringCol};
use crate::database::custom_postgresql::operations::CustomPostgres;
use crate::database::custom_query::CustomQuery;

use crate::models::authors::Authors;
use crate::models::book_vectors_word_normal::BookVectorsWordNormal;
use crate::models::books::Books;
use crate::models::files::Files;

use crate::models::book_vectors_term_2_normal::BookVectorsTerm2Normal;
use crate::models::book_vectors_term_3_normal::BookVectorsTerm3Normal;
use crate::models::book_vectors_term_4_normal::BookVectorsTerm4Normal;
use crate::models::book_vectors_term_5_normal::BookVectorsTerm5Normal;
use crate::models::book_vectors_term_6_normal::BookVectorsTerm6Normal;
use crate::models::book_vectors_term_7_normal::BookVectorsTerm7Normal;


use crate::models::book_vectors_phrase_normal_2::BookVectorsPhraseNormal2;
use crate::models::book_vectors_phrase_normal_3::BookVectorsPhraseNormal3;
use crate::models::book_vectors_phrase_normal_4::BookVectorsPhraseNormal4;

use crate::global::utils;
use crate::global::utils::{
    convert_to_json, convert_to_json_string, count_terms, count_terms_string, get_terms, get_words,
    print_in_file,divide_word,handle_word
};

use std::collections::HashMap;

use std::env::current_dir;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;


use crate::global::stemmer;
use crate::global::stemmer::{PortarsStemmer};

extern crate encoding;

use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::WINDOWS_1251;


extern crate logger;
use logger::Logger;

pub fn add_author(author_full_name: String, postgres_obj: &CustomPostgres) -> i32 {
    //Check author exists
    let mut query = CustomQuery::new();
    query.set_from("authors");
    query.set_condition(&format!("full_name = '{}'", &author_full_name));

    let res = postgres_obj
        .select(&query)
        .expect("Не удалось извлечь авторов");

    let new_auth_id: i32;

    if res.len() == 0 {
        let author = Authors {
            full_name: StringCol::new(&author_full_name),
            ..Authors::default()
        };

        let result = postgres_obj
            .insert(&author)
            .expect("Не удалось добавить автора");

        new_auth_id = result.get(0).get("author_id");
    } else {
        new_auth_id = res.get(0).get("author_id")
    }

    new_auth_id
}

pub fn add_file(path: String, postgres_obj: &CustomPostgres) -> Files {
    let mut file = Files {
        full_name: StringCol::new("parapam"),
        ..Files::default()
    };

    let result = postgres_obj
        .insert(&file)
        .expect("Не удалось создать файл");

    let file_id: i32 = result.get(0).get("file_id");
    file.set_file_id(SomeValue(file_id));

    let mut new_file_name = current_dir().expect("Какая-то проблема с определением текущей директории");

    new_file_name.push("books");

    println!("{}",
        (*new_file_name.as_os_str()).to_str().expect("Не удалось конвертировать в String новый путь к файлу"));

    new_file_name.push(file_id.to_string());
    new_file_name.set_extension("txt");

    println!("{}",
        (*new_file_name.as_os_str()).to_str().expect("Не удалось конвертировать в String новый путь к файлу"));

    let file_name_string = new_file_name.into_os_string().into_string()
                            .expect("Не удалось конвертировать в String новый путь к файлу");
    let some_new = file_name_string.as_str();

    file.set_full_name(SomeValue(some_new.to_owned()));
    postgres_obj
        .update(&file)
        .expect("Не удалось обновить файл");

    //Копирование файла

    println!("source: {}", &path);
    println!("dest: {}", file_name_string);

    fs::copy(path, file_name_string)
        .expect("Какая-то проблема с копирование файла");

    file
}

pub fn add_book(
    book_name: String,
    file: &Files,
    author: Option<&Authors>,
    postgres_obj: &CustomPostgres,
) -> Books {
    panic!("function add_book needs in rewriting");
    // let file_id = match file.get_file_id() {
    //     SomeValue(val) => val,
    //     _ => panic!("Созданный файл не имеет id"),
    // };

    // let mut book = Books {
    //     name: StringCol::new(&book_name),
    //     file_id: IntCol::new(*file_id),
    //     ..Books::default()
    // };
    // match author {
    //     Some(var) => {
    //         match var.get_author_id() {
    //             SomeValue(val) => {
    //                 book.set_author_id(SomeValue(*val));
    //             }
    //             _ => {}
    //         };
    //     }
    //     None => {}
    // }

    // let book_id = postgres_obj.insert_model(&book);

    // book.set_book_id(SomeValue(book_id));

    // book
}

pub fn get_clipped_words(text: &str)->Vec<String>{
    let words = get_words(text);
    // let words = text.split_whitespace();

    let mut clipped_words:Vec<String> = Vec::new();

    let stemmer = PortarsStemmer::new();
    for word in &words {
        // let divided_words = divided_words(word,[",",".",":",";","?","!"]);
        if word.trim().len()>0{

            let some = stemmer::clip(&word.to_lowercase(),&stemmer);

            clipped_words.push(some);
        }
    }

    clipped_words
}

pub fn get_clipped_words_with_punctuation(text: &str)->Vec<String>{
    // let words = get_words(text);
    let words:Vec<&str> = text.split_whitespace().collect();

    let mut clipped_words:Vec<String> = Vec::new();

    let punctuation = vec![",",".",":",";","?","!","—","&","#","[","]","(",")","{","}"];

    let stemmer = PortarsStemmer::new();
    for word in &words {
        let divided_words = divide_word(word,&punctuation, None);
        for word in divided_words {
            let word = word.trim().to_string();
            if word.len()>0{
                if punctuation.contains(&word.as_str()) {
                    match clipped_words.pop() {
                        Some(last_word) => {
                            clipped_words.push(last_word + &word);
                        },
                        None => {}
                    }
                } else {
                    let some: String = stemmer::clip(&word.to_lowercase(),&stemmer);

                    if some != "" {
                        clipped_words.push(some);
                    }
                }
            }
        }
    }

    clipped_words
}

pub fn get_phrases(text: &str, phrases_len: usize)->Vec<String>{
    let words = get_clipped_words_with_punctuation(text);

    let mut phrases:Vec<String> = Vec::new();

    let mut count: usize = 0;
    let mut last_phrase = String::new();
    let mut found;
    for word in words {
        found = false;
            if word.ends_with(|c : char| c.is_ascii_punctuation()) {
                if word.chars().any(|c : char| c.is_alphabetic()) {
                    if count > 0 {
                        last_phrase.push(' ');
                    }
                    last_phrase.push_str(&handle_word(&word));
                    phrases.push(last_phrase);
                }

                count = 0;
                last_phrase = String::new();
                found = true;
            }

        if !found {
            if count > 0 {
                last_phrase.push(' ');
            }
            last_phrase.push_str(&handle_word(&word));
            count+=1;

            if count == phrases_len {
                phrases.push(last_phrase);
                count = 0;
                last_phrase = String::new();
            }
        }
    }

    phrases
}

pub fn read_ansi_file(file_name: &str) -> Option<String> {
    let res = OpenOptions::new()
        .read(true).open(file_name);

    let mut file = match res {
        Ok(file) => file,
        Err(_) => {return None;}
    };

    let mut reader = BufReader::new(&file);

    let mut buf: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buf).expect("Не удалось прочитать файл");

    let mut logger = Logger::new("log_ascii_reading");

    logger.write_bytes(buf.as_slice());

    Some(WINDOWS_1251.decode(buf.as_slice(),DecoderTrap::Strict).unwrap())
}

pub fn read_file(file_name: &str) -> String {
    match fs::read_to_string(file_name){
        Ok(data) => {
            data
        },
        Err(_) => {
            read_ansi_file(file_name).unwrap()
        }
    }
}

pub fn save_words_into_database(
    book_id: i32,
    data: &str,
    postgres_obj: &CustomPostgres,
) -> BookVectorsWordNormal {
    let mut book_vector = BookVectorsWordNormal {
        book_id: IntCol::new(book_id),
        ..BookVectorsWordNormal::default()
    };

    let words = get_clipped_words(data);

    let counted_terms = count_terms_string(words);
    
    let temp_len: f64 = utils::get_vector_len_string(&counted_terms);

    book_vector.set_length(SomeValue(temp_len));

    let mut recounted_terms = HashMap::new();

    for (term, count) in counted_terms {
        recounted_terms.insert(term, ((count as f32) / temp_len as f32) as f32);
    }

    let json = convert_to_json_string(&recounted_terms);

    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open("log.txt")
    //     .expect("Не удалось открыть log.txt");

    // print_in_file(&json, &mut file);

    book_vector.set_vector_direct(SomeValue(json));

    postgres_obj
        .update(&book_vector)
        .expect("Unable insert book vector");

    book_vector
}

pub fn save_phrases_into_database(
    book_id: i32,
    data: &str,
    phrases_len: usize,
    postgres_obj: &CustomPostgres,
) {
    //Тут все дублируется. Надо будет как-нибудь переделать это через трейты
    match phrases_len {
        2 => {
            let mut book_vector = BookVectorsPhraseNormal2 {
                book_id: IntCol::new(book_id),
                ..BookVectorsPhraseNormal2::default()
            };

            let phrases = get_phrases(data, phrases_len);

            let counted_terms = count_terms_string(phrases);
    
            let temp_len: f64 = utils::get_vector_len_string(&counted_terms);

            book_vector.set_length(SomeValue(temp_len));

            let mut recounted_terms = HashMap::new();

            for (term, count) in counted_terms {
                recounted_terms.insert(term, ((count as f32) / temp_len as f32) as f32);
            }

            let json = convert_to_json_string(&recounted_terms);

            book_vector.set_vector_direct(SomeValue(json));

            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        },
        3 => {
            let mut book_vector = BookVectorsPhraseNormal3 {
                book_id: IntCol::new(book_id),
                ..BookVectorsPhraseNormal3::default()
            };

            let phrases = get_phrases(&data, phrases_len);

            let counted_terms = count_terms_string(phrases);
    
            let temp_len: f64 = utils::get_vector_len_string(&counted_terms);

            book_vector.set_length(SomeValue(temp_len));

            let mut recounted_terms = HashMap::new();

            for (term, count) in counted_terms {
                recounted_terms.insert(term, ((count as f32) / temp_len as f32) as f32);
            }

            let json = convert_to_json_string(&recounted_terms);

            book_vector.set_vector_direct(SomeValue(json));

            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        },
        4 => {
            let mut book_vector = BookVectorsPhraseNormal4 {
                book_id: IntCol::new(book_id),
                ..BookVectorsPhraseNormal4::default()
            };

            let phrases = get_phrases(&data, phrases_len);

            let counted_terms = count_terms_string(phrases);
    
            let temp_len: f64 = utils::get_vector_len_string(&counted_terms);

            book_vector.set_length(SomeValue(temp_len));

            let mut recounted_terms = HashMap::new();

            for (term, count) in counted_terms {
                recounted_terms.insert(term, ((count as f32) / temp_len as f32) as f32);
            }

            let json = convert_to_json_string(&recounted_terms);

            book_vector.set_vector_direct(SomeValue(json));

            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        },
        _ => panic!("Такое количество не поддерживается"),
    }
} 

pub fn save_terms_into_database(
    book_id: i32,
    data: &str,
    term_len: usize,
    postgres_obj: &CustomPostgres,
) {

    match term_len {
        2 => {
            let mut book_vector = BookVectorsTerm2Normal {
                book_id: IntCol::new(book_id),
                ..BookVectorsTerm2Normal::default()
            };

            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        },
        3 => {
            let mut book_vector = BookVectorsTerm3Normal {
                book_id: IntCol::new(book_id),
                ..BookVectorsTerm3Normal::default()
            };

            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        },
        4 => {
            let mut book_vector = BookVectorsTerm4Normal {
                book_id: IntCol::new(book_id),
                ..BookVectorsTerm4Normal::default()
            };

            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        },
        5 => {
            let mut book_vector = BookVectorsTerm5Normal {
                book_id: IntCol::new(book_id),
                ..BookVectorsTerm5Normal::default()
            };

            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        },
        6 => {
            let mut book_vector = BookVectorsTerm6Normal {
                book_id: IntCol::new(book_id),
                ..BookVectorsTerm6Normal::default()
            };


            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        },
        7 => {
            let mut book_vector = BookVectorsTerm7Normal {
                book_id: IntCol::new(book_id),
                ..BookVectorsTerm7Normal::default()
            };

            postgres_obj
                .update(&book_vector)
                .expect("Unable insert book vector");
        }
        _ => panic!("Такое количество не поддерживается"),
    }
}

fn get_json_for_terms(data: String, term_len: usize) -> (String,f64) {
    let terms = get_terms(&data, term_len);

    let counted_terms = count_terms_string(terms);

    let temp_len: f64 = utils::get_vector_len_string(&counted_terms);

    let mut recounted_terms = HashMap::new();

    for (term, count) in counted_terms {
        recounted_terms.insert(term, ((count as f32) / temp_len as f32) as f32);
    }

    (convert_to_json_string(&recounted_terms),temp_len)
}
