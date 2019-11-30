extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate postgres;

extern crate magister_add_book;

extern crate ini;

extern crate clap;
use clap::{Arg, App, SubCommand};

use std::env::args;

use magister_add_book::global::manager::Manager;
use magister_add_book::global::database_manager::DatabaseManager;

use std::time::Instant;
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
use magister_add_book::models::book_vectors_term_7_normal::BookVectorsTerm7Normal;
use magister_add_book::models::book_vectors_phrase_normal_3::BookVectorsPhraseNormal3;
use magister_add_book::models::book_vectors_phrase_normal_2::BookVectorsPhraseNormal2;
use magister_add_book::models::book_vectors_phrase_normal_4::BookVectorsPhraseNormal4;
use magister_add_book::models::books::Books;
use magister_add_book::models::files::Files;
use magister_add_book::models::genres::Genres;
use magister_add_book::models::genres_books::GenresBooks;
use magister_add_book::models::stats::Stats;
use magister_add_book::database::common::ModelInfo;

use std::collections::HashMap;

use magister_add_book::add_book::add_book;

use magister_add_book::global::utils;
use magister_add_book::global::utils::*;
use magister_add_book::global::vectors;

use magister_add_book::global::stemmer;
use magister_add_book::global::stemmer::{PortarsStemmer};

use ini::Ini;
use ini::ini::Properties;

extern crate logger;

use logger::Logger;

// pub struct Record {
//     id : Option<i32>,
//     name : Option<String>
// }

const NAME_CONFIG: &str = "config.ini";

fn main() {
    let manager = Manager::new(1);

    let mut action = String::new();
    let mut second_par = String::new();
    let mut third_arg = String::new();

    // if manager.isDebug() {
    //     // action = "add".to_owned();
    //     // action = "select".to_owned();
    //     action = "test".to_owned();
    //     second_par = "books_file.txt".to_owned();
        
    // } else if args().len() < 2 {
    //     panic!("Too few argumants. You must to use 1")
    // } else {
    //     let mut index = 0;
    //     for arg in args() {
    //         if index == 1 {
    //             action = arg;
    //         } else if index == 2 {
    //             second_par = arg;
    //         } else if index == 3 {
    //             third_arg = arg;
    //         }

    //         index = index + 1;
    //     }
    // }

    let matches = App::new("Input params generator")
                          .version("1.0")
                          .author("Titow German")
                          .about("Program to make some difficuls things with books")
                          .arg(Arg::with_name("mode")
                                .short("m")
                                .long("mode")
                                .help("Mode of the work. Now you can use following: add, add-from-db, select, test, 
                                gen-for-genres, gen-for_authors, gen-stat, fill-books-stats")
                                .value_name("MODE"))
                          .arg(Arg::with_name("second")
                                .short("s")
                                .long("second")
                                .help("second parameter")
                                .value_name("SECOND"))
                          .arg(Arg::with_name("third")
                                .short("t")
                                .long("third")
                                .help("third parameter")
                                .value_name("THIRD"))
                          .get_matches();

    let action = match matches.value_of("mode") {
        Some(mode) => {
            mode.to_string()
        },
        None => {
            panic!("You need to specify mode!");
        }
    };

    second_par = match matches.value_of("second") {
        Some(par) => {
            par.to_string()
        },
        None => {
            String::new()
        }
    };

    third_arg = match matches.value_of("third") {
        Some(par) => {
            par.to_string()
        },
        None => {
            String::new()
        }
    };

    let database_manager = DatabaseManager::new(0, 
                                            "postgres".to_owned(), 
                                            "1234".to_owned(), 
                                            "magister_books".to_owned(), 
                                            None);

    match action.as_str() {
        "add" => {
            add_books_from_file(&second_par, database_manager);
        },
        "add-from-db" => {

            let conf_res = Ini::load_from_file(NAME_CONFIG);

            let conf = match conf_res {
                Ok(ok) => ok,
                Err(err) => panic!(format!("You need to create ini file \"{}\"",NAME_CONFIG)),
            };

            let book_addition_section = conf
                        .section(Some("bookAddition".to_owned()))
                        .expect("You need to add section \"bookAddition\" in configuration file");

            let last_id: i32 = book_addition_section
                        .get("last_id")
                        .expect("You need to add \"last_id\" in section \"Controllers\"")
                        .parse::<i32>().unwrap();

            let max_books: i32 = book_addition_section
                        .get("max_books")
                        .expect("You need to add \"last_id\" in section \"Controllers\"")
                        .parse::<i32>().unwrap();

            let books_map: HashMap<i32,Files> = HashMap::new();
            parse_books_from_db(last_id, 
                                Some(max_books),
                                database_manager);
        }
        "select"=>{
            //-------------------------------------------------------------------
            println!("second_par = #{}#", second_par);
            let favs = vec![second_par.parse::<i32>().unwrap()];
            //-------------------------------------------------------------------

            select_books(favs, database_manager);
        },
        "test" => {
            // let words = vec!["противоестественном", "производственный", "дом", "лиса", "завод", "угол", "вонючка"];

            // let words = vec!["одно", "ужасное", "событие", "испортило", "окончание", "этой", "недели", "воскресенье", "пошел", "дождь"];
            // let stemmer = PortarsStemmer::new();
            // for word in words {
            //     // println!("Исходное слово: {0}, урезанное: {1}", word, stemmer::clip(word,&stemmer));
            //     print!("{0} ", stemmer::clip(word,&stemmer));
            // }

            let text = "—И всё-таки, согласитесь, это даже забавно: вот так встречаться и беседовать друг с другом высоко над землёй.

            Говоривший с улыбкой опустил взгляд. Там, внизу, еле заметные с такой высоты, тянулись тоненькие цепочки огней, сплетаясь в несимметричный узор городских улиц и кварталов.
            
            — Не вижу ничего забавного. Знаете, я бы с большим удовольствием отказался от всяческих выходов в астрал и тому подобной оккультной гимнастики. К сожалению, для меня это самый быстрый способ связи… Хотя и не самый удобный. Сейчас вообще сложное время, вы в курсе — происходит смена зодиака, старые расчёты утрачивают силу, новые ещё не откорректированы, а наши корифеи, как водится, не могут между собой договориться о самых элементарных вещах. Не понимаю, откуда у этих мальчишек такой апломб!
            
            — Мальчишек?";

            let word = "и";
            let stemmer = stemmer::PortarsStemmer::new();
            let res = stemmer::clip(word, &stemmer);
            println!("{}", res);


            let words = add_book::get_phrases(text, 4);

            for word in words {
                print!("{0}, ", word);
            }
        },
        "gen-for-genres" => {
            gen_vectors_for_genres(database_manager);
        },
        "gen-for-authors" => {
            gen_vectors_for_authors(database_manager);
        },
        "gen-stat" => {
            gen_vectors_for_stats(database_manager);
        },
        "fill-books-stats" => {
            fill_book_stats(database_manager)
        },
        _ => panic!("Wrong mode")
    };
}

fn add_books_from_file(file_path : &str, database_manager: DatabaseManager){
    
    panic!("Эта часть недопилина!!!!");

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

        let path: String = file.get_full_name().unwrap();

        if path == "" {
            panic!("Нет полного имени файла!!!");
        }

        // let book = add_book::add_book(book_name, path, author, &postgres_obj);

        // add_book::save_words_into_database(*book.get_book_id().unwrap_ref(),path,&postgres_obj);

        // add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),path,2,&postgres_obj);
        // add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),path,3,&postgres_obj);
        // add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),path,4,&postgres_obj);
        // add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),path,5,&postgres_obj);
        // add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),path,6,&postgres_obj);
        // add_book::save_terms_into_database(*book.get_book_id().unwrap_ref(),path,7,&postgres_obj);
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

    let mut book_id = 0;
    for id in fav_book_ids {
        fav_books_ids_str.push(id.to_string());
        book_id = id;
    }

    let mut arr_ids = utils::union_to_string(&fav_books_ids_str, ",", None);

    arr_ids = "{".to_owned() + &arr_ids + "}";

    // let query = CustomQuery {
    //     from: Some(BookVectorsWordNormal::get_source()),
    //     condition: Some("book_id = ANY(".to_owned() + &arr_ids + ")"),
    //     ..CustomQuery::default()
    // };

    let mut query = CustomQuery::new();
    query.set_from(&BookVectorsPhraseNormal4::get_source());
    query.set_condition(&("book_id = ANY('".to_owned() + &arr_ids + "')"));


    let rows = postgres_obj.select(&query).expect("Не удалось получить предпочтительные книги");

    let mut fav_book_vectors:Vec<BookVectorsPhraseNormal4> = Vec::new();

    for row in rows.iter() {
        let direct_vector: String = row.get("vector_direct");
        fav_book_vectors.push(
            BookVectorsPhraseNormal4{
                book_id:IntCol::new(row.get("book_id")),
                vector_direct: StringCol::new(&direct_vector),
                length: FloatCol64::new(row.get("length")),
                ..BookVectorsPhraseNormal4::default()
            }
        );
    }

    let mut path = String::new();

    path.push_str("test/");
    path.push_str(&book_id.to_string());
    path.push_str("_phrase_4.txt");
    let mut logger = Logger::new(&path);

    logger.writeln("Получил предпочтительные:");

    for fav_book in &fav_book_vectors {
        logger.writeln(&(fav_book.get_book_id().unwrap_ref().to_string()));
    }

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
    
    // utils::output_hash_map(&book_vector_sum, &mut logger);

    // let len = utils::get_vector_len_string(&book_vector_sum);
    // println!("Длина вектора = {}",&len.to_string());

    let mut results: HashMap<i32,f32> = HashMap::new();

    let max = 10;

    println!("{}", max.to_string());

    let mut page: usize = 0;
    let page_size: usize = 100;

    query.set_condition("");
    query.set_limit(&page_size.to_string());

    let start = Instant::now();

    let mut end = false;
    while !end {
        query.set_offset(&(page*page_size).to_string());
        // Получить книги
        println!("Начал запрос");
        let rows = postgres_obj.select(&query).expect("Не удалось получить все книги");
        println!("Выполнил запрос");
        let mut book_vectors:Vec<BookVectorsPhraseNormal4> = Vec::new();

        if rows.len() < page_size {
            end = true;
        }

        for row in rows.iter() {
            let direct_vector: String = row.get("vector_direct");
            book_vectors.push(
                BookVectorsPhraseNormal4{
                    book_id:IntCol::new(row.get("book_id")),
                    vector_direct: StringCol::new(&direct_vector),
                    length: FloatCol64::new(row.get("length")),
                    ..BookVectorsPhraseNormal4::default()
                }
            );
        }

        // Выполнить сравнение

        let mut index = 0;
        let count = book_vectors.len();
        for vector in &book_vectors {
            let book_vector_map: HashMap<String, f32> = serde_json::from_str(vector.get_vector_direct().unwrap_ref().as_str()).unwrap();

            let cos = vectors::get_cos_normal(&book_vector_sum, &book_vector_map);
            // logger.writeln(&format!("book_id = {}, cos = {}",
            //                 vector.get_book_id().unwrap_ref().to_string(),
            //                 cos.to_string()));
            
            index = index + 1;

            println!("{} из {}",index.to_string(), count.to_string());

            if results.len() < max {
                results.insert(*vector.get_book_id().unwrap_ref(), cos);
            } else {
                replace_min(&mut results, cos, *vector.get_book_id().unwrap_ref());
            }
        }
        page = page + 1;

        println!("Закончил со страницей: {}",page.to_string());
    }
    let elapsed = start.elapsed();
    logger.writeln(&format!("Время выполнения (ms): {}",elapsed.as_millis().to_string()));
    logger.writeln("Лучшие 10: ");
    let mut index = 0;
    let count = results.len();
    let mut used_books = vec![];
    while index < count {
        let mut cur_max: f32 = -1.0;
        let mut cur_max_book_id = 0;
        for (book_id,cos) in &results {
            if (cur_max == -1.0 || cur_max < *cos) && !used_books.contains(book_id) {
                cur_max = *cos;
                cur_max_book_id = *book_id;
            }
        }

        used_books.push(cur_max_book_id);

        let mut query = CustomQuery::new();
        query.set_columns("b.name, a.author_id as author_id, a.full_name, 
            array((Select genre_name from genres g inner join genres_books g_b using(genre_id) 
                    where g_b.book_id = b.book_id)) as genres");
            query.set_from("books b inner join authors a using(author_id)");
            query.set_condition(&("b.book_id = ".to_owned()+&cur_max_book_id.to_string()));

        let rows = postgres_obj.select(&query).expect(&format!("Не удалось заполучить полные данные книги {}",cur_max_book_id.to_string()));

        let mut book_name:String = String::new();
        let mut author_name:String = String::new();
        let mut author_id:i32 = 0;
        let mut genres:Vec<String> = Vec::new();
        for row in rows.iter() {
            book_name = row.get("name");
                author_name = row.get("full_name");
                author_id = row.get("author_id");
                genres = row.get("genres");
            }

            let genres_str = union_to_string(&genres, ",", None);

            logger.writeln(&format!("book_id = {}, cos = {}, название = \'{}\', автор = {}, id = {}, жанры = {}",
                cur_max_book_id.to_string(),cur_max.to_string(),book_name,author_name,author_id.to_string(),genres_str));
        
        index+=1;
    }
    

    results
}

fn replace_min(vec: &mut HashMap<i32, f32>, value : f32, key: i32) {
    if vec.len() < 1 {
        panic!("Вектор должен быть не пустой");
    }

    let mut min = 0;
    for (key,item) in &*vec {
        if min == 0 {
            min = *key;
            continue;
        }

        if *item < *(vec.get(&min).unwrap()) {
            min = *key;
        }
    }

    if *vec.get(&min).unwrap() < value {
        vec.insert(key, value);
        vec.remove(&min);
    }
}

fn parse_books_from_db(last_book_id: i32, max: Option<i32>, database_manager: DatabaseManager) {
    let postgres_obj = database_manager.into_postgresql();

    let mut query = CustomQuery::new();

    query.set_columns("f.path_to, b.book_id");
    query.set_from("books b inner join books_files b_f USING (book_id)
                    inner join files f USING (file_id)");
    query.set_condition(&format!("f.extension = \'txt\' and b.book_id > {}",
                                    last_book_id.to_string()));
    query.set_order("book_id asc");
    
    match max {
        Some(max) => {
            query.set_limit(&max.to_string());
        },
        None=>{
        }
    };

    let results = postgres_obj.select(&query).expect("Не удалось получить книги из базы");

    let mut last_book_id: i32 = 0;
    for row in &results {
        let book_id: i32 = row.get("book_id");
        let path: String = row.get("path_to");

        let res = add_book::read_ansi_file(&path);

        let data: String;
        match res {
            Some(_data)=> {data = _data;},
            None => {continue;}
        };

        // add_book::save_words_into_database(book_id,&data,&postgres_obj);

        // add_book::save_terms_into_database(book_id,&data,2,&postgres_obj);
        // add_book::save_terms_into_database(book_id,&data,3,&postgres_obj);
        // add_book::save_terms_into_database(book_id,&data,4,&postgres_obj);
        // add_book::save_terms_into_database(book_id,&data,5,&postgres_obj);
        // add_book::save_terms_into_database(book_id,&data,6,&postgres_obj);
        // add_book::save_terms_into_database(book_id,&data,7,&postgres_obj);

        add_book::save_phrases_into_database(book_id,&data,2, &postgres_obj);
        add_book::save_phrases_into_database(book_id,&data,3, &postgres_obj);
        add_book::save_phrases_into_database(book_id,&data,4, &postgres_obj);

        println!("completed with book {}", book_id.to_string());

        last_book_id = book_id;
    }

    let mut conf = Ini::new();

    conf.with_section(Some("bookAddition".to_owned()))
        .set("last_id".to_owned(), last_book_id.to_string());

    match max {
        Some(max) => {
            conf.with_section(Some("bookAddition".to_owned()))
                .set("max_books".to_owned(), max.to_string());
            },
        None => {}
    };

    conf.write_to_file(NAME_CONFIG).expect("Unable to rewrite config file");
}

fn gen_vectors_for_genres(database_manager: DatabaseManager) {
    let postgres_obj = database_manager.into_postgresql();

    let mut query = CustomQuery::new();

    query.set_from("genres where exists (select * from genres_books where genres_books.genre_id = genres.genre_id)");
    query.set_order("genre_id asc");
    let rows = postgres_obj.select(&query).expect("Unable to get unempty genres from db");
    let mut genres:Vec<Genres> = Vec::new();

    //fill genres from db
    for row in rows.iter() {
        let genre_name:String = row.get("genre_name");
        genres.push(Genres{
            genre_id: IntCol::new(row.get("genre_id")),
            genre_name: StringCol::new(&genre_name),
            ..Genres::default()
        });
    }

    let mut books_count = 0;

    //to each genre add vector
    for mut genre in genres {

        println!("Жанр: {}, с id = {}",genre.get_genre_name().unwrap_ref().to_string(), genre.get_genre_id().unwrap_ref().to_string());
        let mut page = 0;
        let mut page_size = 10;

        let mut end = false;

        let mut middle_vector : HashMap<String, f32> = HashMap::new();

        query.set_from("genres_books g_b inner join books b ON(g_b.book_id = b.book_id) 
                                            inner join book_vectors_word_normal b_vec ON (b.book_id = b_vec.book_id)");
        query.set_columns("b_vec.*");
        query.set_condition(&format!("g_b.genre_id = {}", genre.get_genre_id().unwrap_ref().to_string()));
        
        query.set_limit(&page_size.to_string());

        let mut count;

        //Вычислили средний вектор
        while !end {
            print!("Страница: {}",page.to_string());
            count = 0;
            let offset = page*page_size;
            query.set_offset(&offset.to_string());

            let rows = postgres_obj.select(&query).expect("Unable to get books vectors from db");

            for row in rows.iter() {
                let direct_vector: String = row.get("vector_direct");
                if middle_vector.len() == 0 {
                    middle_vector = serde_json::from_str(&direct_vector).unwrap();
                } else {
                    middle_vector = vectors::concate_and_normalize(&middle_vector, 
                            &serde_json::from_str(&direct_vector).unwrap());
                }
                count = count + 1;
            }

            books_count += count;
            println!(", обработано книг: {}",books_count.to_string());
            page = page + 1;
            if count < page_size {
                end = true;
            }
        }

        let json = convert_to_json_string(&middle_vector);
        genre.set_vector(SomeValue(json));

        postgres_obj.update(&genre).expect("Unable to update genre");
    }
}

fn gen_vectors_for_authors(database_manager: DatabaseManager) {
    let postgres_obj = database_manager.into_postgresql();

    let mut query = CustomQuery::new();

    query.set_from("authors");
    
    let rows = postgres_obj.select(&query).expect("Unable to get authors from db");
    let mut authors:Vec<Authors> = Vec::new();

    //fill authors from db
    for row in rows.iter() {
        let full_name:String = row.get("full_name");
        authors.push(Authors{
            author_id: IntCol::new(row.get("author_id")),
            full_name: StringCol::new(&full_name),
            ..Authors::default()
        });
    }

    let mut books_count = 0;

    //to each author add vector
    for mut author in authors {

        println!("Автор: {}, с id = {}",author.get_author_id().unwrap_ref().to_string(), author.get_full_name().unwrap_ref().to_string());

        let mut middle_vector : HashMap<String, f32> = HashMap::new();

        query.set_from("books b inner join book_vectors_word_normal b_vec ON (b.book_id = b_vec.book_id)");
        query.set_columns("b_vec.*");
        query.set_condition(&format!("b.author_id = {}", author.get_author_id().unwrap_ref().to_string()));

        let mut count = 0;

        //Вычисляем средний вектор

            let rows = postgres_obj.select(&query).expect("Unable to get books vectors from db");

            for row in rows.iter() {
                let direct_vector: String = row.get("vector_direct");
                if middle_vector.len() == 0 {
                    middle_vector = serde_json::from_str(&direct_vector).unwrap();
                } else {
                    middle_vector = vectors::concate_and_normalize(&middle_vector, 
                            &serde_json::from_str(&direct_vector).unwrap());
                }
                count = count + 1;
            }

            books_count += count;
            println!(", обработано книг: {}",books_count.to_string());

        let json = convert_to_json_string(&middle_vector);
        author.set_vector(SomeValue(json));

        postgres_obj.update(&author).expect("Unable to update author");
    }
}

fn gen_vectors_for_stats(database_manager: DatabaseManager) {
    let postgres_obj = database_manager.into_postgresql();

    let mut query = CustomQuery::new();

    //worst
    let mut middle_vector : HashMap<String, f32> = HashMap::new();

        query.set_from("books b inner join book_vectors_word_normal b_vec ON (b.book_id = b_vec.book_id)");
        query.set_columns("b_vec.*");
        query.set_condition(&format!("b.rating_parsed < 2.5"));

        //Вычисляем средний вектор

            let rows = postgres_obj.select(&query).expect("Unable to get books vectors from db");

            for row in rows.iter() {
                let direct_vector: String = row.get("vector_direct");
                if middle_vector.len() == 0 {
                    middle_vector = serde_json::from_str(&direct_vector).unwrap();
                } else {
                    middle_vector = vectors::concate_and_normalize(&middle_vector, 
                            &serde_json::from_str(&direct_vector).unwrap());
                }
            }

        let json = convert_to_json_string(&middle_vector);

        let mut stat = Stats {
            data: StringCol::new(&json),
            id : IntCol::new(1),
            description: StringCol::new("worst"),
            // ..Stats::default()
        };

        postgres_obj.insert(&stat).expect("Unable to insert worst stat");
        

        //best
        query.set_condition(&format!("b.rating_parsed = 5.0"));

        middle_vector = HashMap::new();

        //Вычисляем средний вектор

            let rows = postgres_obj.select(&query).expect("Unable to get books vectors from db");

            for row in rows.iter() {
                let direct_vector: String = row.get("vector_direct");
                if middle_vector.len() == 0 {
                    middle_vector = serde_json::from_str(&direct_vector).unwrap();
                } else {
                    middle_vector = vectors::concate_and_normalize(&middle_vector, 
                            &serde_json::from_str(&direct_vector).unwrap());
                }
            }

        let json = convert_to_json_string(&middle_vector);

        let mut stat = Stats {
            data: StringCol::new(&json),
            id : IntCol::new(2),
            description: StringCol::new("best"),
            // ..Stats::default()
        };

        postgres_obj.insert(&stat).expect("Unable to insert best stat");
}

fn fill_book_stats(database_manager: DatabaseManager) {
    let postgres_obj = database_manager.into_postgresql();

    //get genres
    let mut query = CustomQuery::new();

    query.set_from("genres where exists (select * from genres_books where genres_books.genre_id = genres.genre_id)");
    query.set_order("genre_id asc");

    let rows = postgres_obj.select(&query).expect("Unable to get unempty genres from db");
    let mut genres:Vec<Genres> = Vec::new();

    //fill genres from db
    for row in rows.iter() {
        let genre_name:String = row.get("genre_name");
        let vector: String = row.get("vector");
        let genre_name_english:String = row.get("genre_name_english");
        genres.push(Genres{
            genre_id: IntCol::new(row.get("genre_id")),
            genre_name: StringCol::new(&genre_name),
            genre_name_english: StringCol::new(&genre_name_english),
            vector: StringCol::new(&vector),
            ..Genres::default()
        });
    }

    println!("Получил жанры");

    //get stats
    let mut query = CustomQuery::new();

    query.set_from("stats");
    
    let rows = postgres_obj.select(&query).expect("Unable to get stats from db");
    let mut stats:Vec<Stats> = Vec::new();

    //fill stats from db
    for row in rows.iter() {
        let data:String = row.get("data");
        let description: String = row.get("description");
        stats.push(Stats{
            id: IntCol::new(row.get("id")),
            data: StringCol::new(&data),
            description: StringCol::new(&description),
            ..Stats::default()
        });
    }
    println!("Получил статистику");

    let mut query = CustomQuery::new();

    query.set_from("book_vectors_word_normal b_vec inner join books using(book_id) inner join authors using (author_id)");
    query.set_order("book_id asc");

    let mut books_count = 0;
    let mut page = 0;
    let mut page_size = 10;

    query.set_limit(&page_size.to_string());

    let mut end = false;
    let mut count = 0;
    
    println!("Дошел до цикла");
        while !end {
            print!("Страница: {}",page.to_string());
            count = 0;
            let offset = page*page_size;
            query.set_offset(&offset.to_string());

            let rows = postgres_obj.select(&query).expect("Unable to get books vectors from db");
            
            for row in rows.iter() {
                let book_id: i32 = row.get("book_id");
                let author_vector: String = row.get("vector");
                let book_vector: String = row.get("vector_direct");

                let mut insert_query = String::from("INSERT INTO books_stats");
                let mut insert_values = String::from("VALUES(");
                let mut insert_columns = String::from("(");

                insert_values.push_str(&book_id.to_string());
                insert_columns.push_str("book_id");

                //разница с автором
                let vector_author_map: HashMap<String, f32> = serde_json::from_str(&author_vector).unwrap();
                let book_vector_map: HashMap<String, f32> = serde_json::from_str(&book_vector).unwrap();

                let cos_author = vectors::get_cos_normal(&book_vector_map, &vector_author_map);

                insert_columns.push_str(", ");
                insert_values.push_str(", ");

                insert_columns.push_str("diff_author");
                insert_values.push_str(&cos_author.to_string());

                //разница с жанрами
                // println!("Начал перебирать жанры");

                for genre in &genres {
                    let vector_genre_map: HashMap<String, f32> = serde_json::from_str(
                        &genre.get_vector().unwrap_ref().to_string()).unwrap();

                    // println!("Распарсил вектор жанра {}",genre.get_genre_name().unwrap_ref().to_string());

                    let cos_genre = vectors::get_cos_normal(&book_vector_map, &vector_genre_map);

                    insert_columns.push_str(", ");
                    insert_values.push_str(", ");

                    insert_columns.push_str("diff_genre_");

                    let mut genre_name = genre.get_genre_name_english().unwrap_ref().to_string();
                    //println!("Название жанра на английском: {}", genre_name);
                    genre_name = genre_name.replace(" ", "_");
                    genre_name = genre_name.replace("\'", "_");
                    genre_name = genre_name.replace("(", "_");
                    genre_name = genre_name.replace(")", "_");
                    genre_name = genre_name.replace(",", "_");

                    insert_columns.push_str(&genre_name);
                    insert_values.push_str(&cos_genre.to_string());
                    //println!("Закончил с жанром");
                }

                //разница со статистикой
                //println!("Начал перебирать статистику");
                for stat in &stats {
                    let vector_stat_map: HashMap<String, f32> = serde_json::from_str(
                         &stat.get_data().unwrap_ref().to_string()).unwrap();
                    //println!("Распарсил вектор статистики");

                    let cos_stat = vectors::get_cos_normal(&book_vector_map, &vector_stat_map);

                    insert_columns.push_str(", ");
                    insert_values.push_str(", ");

                    insert_columns.push_str("diff_stat_");
                    insert_columns.push_str(&stat.get_description().unwrap_ref().to_string());
                    insert_values.push_str(&cos_stat.to_string());
                    //println!("Закончил со статистикой");
                }

                insert_values.push_str(")");
                insert_columns.push_str(") ");

                insert_query.push_str(&insert_columns);
                insert_query.push_str(" ");
                insert_query.push_str(&insert_values);

                //Запрос написан, все готово
                //println!("Закончил запрос");
                count = count + 1;

                if books_count == 0 && count == 1 {
                    let mut logger = Logger::new("books_stats_log.txt");
                    logger.writeln(&insert_query);
                }

                postgres_obj.execute(&insert_query).expect("unable insert books_stats");
            }

            books_count += count;
            println!(", обработано книг: {}",books_count.to_string());
            page = page + 1;
            if count < page_size {
                end = true;
            }
        }

}