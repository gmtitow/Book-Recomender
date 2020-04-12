extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate postgres;

extern crate magister_add_book;

use std::env::args;

use magister_add_book::global::manager::Manager;
use magister_add_book::global::database_manager::DatabaseManager;

use std::fs;
// use std::fs::File;
// use std::io::Write;
use std::path::Path;
use magister_add_book::models::authors::Authors;
// use magister_add_book::database::custom_postgresql::operations::CustomPostgres;
// use magister_add_book::models::books::Books;

// use magister_add_book::models::files::Files;
use magister_add_book::database::common::{ColEnum,StringCol,IntCol, IntCol64,FloatCol64};
use magister_add_book::database::common::ColEnum::{SomeValue,Null,Nothing};
use magister_add_book::database::custom_query::CustomQuery;
use magister_add_book::models::book_vectors_word_normal::BookVectorsWordNormal;
use magister_add_book::database::common::ModelInfo;

use std::collections::HashMap;

use magister_add_book::add_book::add_book;

use magister_add_book::global::utils;
use magister_add_book::global::vectors;

use magister_add_book::global::stemmer;
use magister_add_book::global::stemmer::{PortarsStemmer};

extern crate logger;

use logger::Logger;

// pub struct Record {
//     id : Option<i32>,
//     name : Option<String>
// }

fn main() {
    let manager = Manager::new(0);

    let mut action = String::new();
    let mut second_par = String::new();

    //--------------------------------------
    let mut fav_books = vec![158];
    //---------------------------------------

    if manager.isDebug() {
        // action = "add".to_owned();
        // action = "select".to_owned();
        action = "test".to_owned();
        second_par = "books_file.txt".to_owned();
        
    } else if args().len() < 2 {
        panic!("Too few argumants. You must to use 1")
    } else {
        let mut index = 0;
        for arg in args() {
            if index == 1 {
                action = arg;
            } else if index == 2 {
                second_par = arg;
                break;
            }

            index = index + 1;
        }
    }

    let database_manager = DatabaseManager::new(0, 
                                            "postgres".to_owned(), 
                                            "1234".to_owned(), 
                                            "magister_books".to_owned(), 
                                            None);

    // fs::copy("E:\\Временное с полетевшего компа\\Книги\\Джоан Роулинг\\Dzhoan_Rouling__Garri_Potter_i_Taynaya_komnata.txt", 
    //         "E:\\books\\".to_owned()+&11.to_string()+".txt")
    //         .expect("Совсем не удалось скопировать Фрейда");

    match action.as_str() {
        "add" => {
            add_books(&second_par, database_manager);
        },
        "select"=>{
            select_books(fav_books, database_manager);
        },
        "test" => {
            let words = vec!["противоестественном", "производственный", "дом", "лиса", "завод", "угол", "вонючка"];

            let stemmer = PortarsStemmer::new();
            for word in words {
                println!("Исходное слово: {0}, урезанное: {1}", word, stemmer::clip(word,&stemmer));
            }
        }
        _ => panic!("Wrong action")
    };
}

fn add_books(file_path : &str, database_manager: DatabaseManager){
    let postgres_obj = database_manager.into_postgresql();

    let path = Path::new(file_path);

    let books_data = fs::read_to_string(path).expect("Не удалось открыть файл с книгами");;

    for line in books_data.lines() {
        if line.len() == 0 {
            continue;
        }

        for _char in line.chars() {
            print!("{}", _char);
        }

        let splite_line = line.split("###");

        let mut book_path = String::new();
        let mut book_name = String::new();
        let mut author_full_name = String::new();

        let mut index = 0;
        for part in splite_line {
            if index == 0 {
                book_path = part.trim().to_owned();
            } else if index == 1 {
                book_name  = part.trim().to_owned();
            } else if index == 2 {
                author_full_name  = part.trim().to_owned();
            } else {
                break
            }

            index = index + 1;
        }

        if book_path == "" || book_name == "" {
            panic!("Не удалось прочесть название или путь к файлу. Это беда.");
        }

        let mut new_auth_id: Option<i32> = None;

        //TODO переработать проверку автора на существование : проверка по полному имени некорректна
        if author_full_name != "" {
            new_auth_id = Some(add_book::add_author(author_full_name, &postgres_obj));
        }
        //Проверка и добавление автора сделаны
        
        //TODO сделать проверку существования книги в базе

        //Создание файла
        let file = add_book::add_file(book_path, &postgres_obj);

        //Создание книги
        
        //TODO удалить этот костыль
        let mut author: Option<&Authors> = None;
        let author_obj: Authors;

        match new_auth_id {
         Some(var) => {
            author_obj  = Authors {
                author_id: IntCol::new(var),
                ..Authors::default()
            };

            author = Some(&author_obj);
         },
         None => {}
        };

        let book = add_book::add_book(book_name, &file, author, &postgres_obj);

        add_book::save_words_into_database(*book.get_book_id().unwrap_ref(),&file,&postgres_obj);

        add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),&file,2,&postgres_obj);
        add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),&file,3,&postgres_obj);
        add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),&file,4,&postgres_obj);
        add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),&file,5,&postgres_obj);
        add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),&file,6,&postgres_obj);
        add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),&file,7,&postgres_obj);
    }

    //Все закончил, закрыл соединение с базой
    let close_res = postgres_obj.close_connection();

    match close_res{
        Ok(_)=>println!("Закрыл соединение - юхуу!!!"),
        Err(err)=>println!("Ошибка при закрытии соединения - {}",err),
    };
}

fn select_books(fav_book_ids: Vec<i32>, database_manager: DatabaseManager)->HashMap<i32,f32> {
    let postgres_obj = database_manager.into_postgresql();

    let mut fav_books_ids_str = Vec::new();

    let mut results: HashMap<i32,f32> = HashMap::new();

    for id in fav_book_ids {
        fav_books_ids_str.push(id.to_string());
    }

    let mut arr_ids = utils::union_to_string(&fav_books_ids_str, ",", None);

    arr_ids = "{".to_owned() + &arr_ids + "}";

    // let query = CustomQuery {
    //     from: Some(BookVectorsWordNormal::get_source()),
    //     condition: Some("book_id = ANY(".to_owned() + &arr_ids + ")"),
    //     ..CustomQuery::default()
    // };

    let mut query = CustomQuery::new();
    query.set_from(&BookVectorsWordNormal::get_source());
    query.set_condition(&("book_id = ANY('".to_owned() + &arr_ids + "')"));


    let rows = postgres_obj.select(&query).expect("Не удалось получить предпочтительные книги");

    let mut fav_book_vectors:Vec<BookVectorsWordNormal> = Vec::new();

    for row in rows.iter() {
        let direct_vector: String = row.get("vector_direct");
        fav_book_vectors.push(
            BookVectorsWordNormal{
                book_id:IntCol::new(row.get("book_id")),
                vector_direct: StringCol::new(&direct_vector),
                length: FloatCol64::new(row.get("length")),
                ..BookVectorsWordNormal::default()
            }
        );
    }

    query.set_condition("");

    let rows = postgres_obj.select(&query).expect("Не удалось получить все книги");

    let mut book_vectors:Vec<BookVectorsWordNormal> = Vec::new();

    for row in rows.iter() {
        let direct_vector: String = row.get("vector_direct");
        book_vectors.push(
            BookVectorsWordNormal{
                book_id:IntCol::new(row.get("book_id")),
                vector_direct: StringCol::new(&direct_vector),
                length: FloatCol64::new(row.get("length")),
                ..BookVectorsWordNormal::default()
            }
        );
    }

    let mut logger = Logger::new("log_books_recommends.txt");

    logger.writeln("Получил предпочтительные:");

    for fav_book in &fav_book_vectors {
        logger.writeln(&(fav_book.get_book_id().unwrap_ref().to_string()));
    }

    logger.writeln("Получил все:");

    for books in &book_vectors {
        logger.writeln(&(books.get_book_id().unwrap_ref().to_string()));
    }

    /*for fav_book in &fav_book_vectors {

        println!("Длина вектора = {}",&fav_book.get_book_id().unwrap_ref());
        match fav_book.get_vector_direct() {
            SomeValue(val) => {
                let book_vector : HashMap<String, f32> = serde_json::from_str(val.as_str()).unwrap();

                let len = utils::get_vector_len_string(&book_vector);
                
                println!("Длина вектора = {}",&len.to_string());

                utils::output_hash_map(&book_vector, &mut logger);
            },
            _=>{}
        };
    }*/

    let mut book_vector_sum : HashMap<String, f32> = HashMap::new();

    for fav_book in &fav_book_vectors {
        match fav_book.get_vector_direct() {
            SomeValue(val) => {
                if book_vector_sum.len() == 0 {
                    book_vector_sum = serde_json::from_str(val.as_str()).unwrap();
                } else {
                    book_vector_sum = vectors::concate_and_normalize(&book_vector_sum, 
                                                                     &serde_json::from_str(val.as_str()).unwrap());
                }
            },
            _=>{}
        };
    }
    
    utils::output_hash_map(&book_vector_sum, &mut logger);

    let len = utils::get_vector_len_string(&book_vector_sum);
    println!("Длина вектора = {}",&len.to_string());

    for vector in &book_vectors {
        print!("book_id = {}",vector.get_book_id().unwrap_ref());

        let book_vector_map: HashMap<String, f32> = serde_json::from_str(vector.get_vector_direct().unwrap_ref().as_str()).unwrap();

        let cos = vectors::get_cos_normal(&book_vector_sum, &book_vector_map);

        println!(", cos = {}",cos);
    }

    results
}
