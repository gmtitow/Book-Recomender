extern crate postgres;
use postgres::{Connection, TlsMode, Error};
use postgres::rows::Rows;
use postgres::stmt::Column;
use postgres::types::ToSql;
use crate::database::common::ModelInfo;
use crate::database::common::ColEnum;
use crate::database::common::QueryResult;
use crate::database::custom_query::CustomQuery;

use crate::global::utils::union_to_string;

extern crate logger;
use logger::Logger;

pub struct CustomPostgres {
    connection: Connection
}

impl CustomPostgres{
    fn create_connection(user: &str, password: &str, database: &str)->Result<Connection,Error> {
        let url = format!("postgresql://{}:{}@localhost/{}",user,password,database);
        Connection::connect(url.to_string(), TlsMode::None)
    }

    fn form_insert_query<'a, T: ModelInfo>(model: &T) -> String{
        let columns = model.get_columns();
        let values = model.get_values();

        let mut query = "INSERT INTO ".to_string();

        query = query + model.get_table_name();

        let mut query_end = String::new();

        let mut first = true;

        for (index,column) in columns.iter().enumerate(){
            match &values[index] {
                Some(val) => {
                    if !first {
                        query += ", ";
                        query_end += ", ";
                    } else {
                        query.push_str(" (");
                        query_end = "VALUES(".to_string();
                        first = false;
                    }

                    query += &column;
                    query_end += &val;
                },
                None =>{}
            }
        }

        if !first {
            query_end+=")";
            query+=")";
        } else {
            query_end = "DEFAULT VALUES".to_owned();
        }

        let prim_keys = union_to_string(&model.get_primary_keys_names(),", ", None);

        let all_query = query+ " " + &query_end + " RETURNING "+&prim_keys+";";

        // println!("insert query: {0}",&all_query);
        let mut logger = Logger::new("insert_log.txt");
        logger.writeln(&all_query);

        all_query
    }

    fn form_update_query<'a, T: ModelInfo>(model: &T) -> String{
        let columns = model.get_columns();
        let values = model.get_values();

        let mut query = "UPDATE ".to_string();

        query = query + model.get_table_name() + " SET ";

        let mut first = true;

        for (index,column) in columns.iter().enumerate(){
            match &values[index] {
                Some(val) => {
                    if !first {
                        query += ", ";
                    } else {
                        first = false;
                    }

                    query.push_str(&format!("{0} = {1}",column,&val));

                },
                None =>{}
            }
        }

        query.push_str(" WHERE ");

        let prim_keys = model.get_primary_keys_names();
        let prim_values = model.get_primary_keys_values();

        for (index,key) in prim_keys.iter().enumerate() {
            match &prim_values[index] {
                Some(val) => {
                    if index!=0 {
                        query.push_str(" AND ");
                    }
                    query.push_str(&format!("{0} = {1}",&key,&val));
                },
                None => {
                    panic!("Пустой первичный ключ при update - нехорошо");
                }
            }
        }

        // println!("update query: {0}",&query);

        query
    }

    fn form_delete_query<T: ModelInfo>(model: &T) -> String{
        let columns = model.get_primary_keys_names();
        let values = model.get_primary_keys_values();

        let mut query = "DELETE FROM ".to_string();

        query = query + model.get_table_name() + " WHERE ";
        let mut first = true;
        for (index,column) in columns.iter().enumerate() {
            match &values[index] {
                Some(val) => {
                    if !first {
                        query += " AND ";
                    } else {
                        first = false;
                    }
                    query = query + column + " = " + &val;
                },
                None =>{panic!("Primary key can't be empty")}
            }
        }
        // println!("query: {0}",&query);
        query
    }

    fn form_select_query(query: &CustomQuery)->String{
        let mut query_string = "SELECT ".to_string();

        match query.get_distinct() {
            Some(val) => query_string.push_str(&val),
            _ => {}
        };

        query_string.push_str(" ");

        match query.get_columns() {
            Some(val) => query_string.push_str(&val),
            None => query_string.push_str("*")
        };

        query_string.push_str(" FROM ");

        match query.get_from() {
            Some(val) => query_string.push_str(&val),
            None => panic!("Не указано поле FROM (обязательное и незаменимое ничем)")
        };
        
        match query.get_condition() {
            Some(val) => {
                if val.trim() != ""{
                    query_string.push_str(" WHERE ");
                    query_string.push_str(&val)
                }
            },
            None => {}
        };

        match query.get_order() {
            Some(val) => {
                query_string.push_str(" ORDER BY ");
                query_string.push_str(&val)
            },
            None => {}
        };

        match query.get_group() {
            Some(val) => {
                query_string.push_str(" GROUP BY ");
                query_string.push_str(&val)
            },
            None => {}
        };

        match query.get_group() {
            Some(val) => {
                query_string.push_str(" GROUP BY ");
                query_string.push_str(&val)
            },
            None => {}
        };

        match query.get_limit() {
            Some(val) => {
                query_string.push_str(" LIMIT ");
                query_string.push_str(&val)
            },
            None => {}
        };

        match query.get_offset() {
            Some(val) => {
                query_string.push_str(" OFFSET ");
                query_string.push_str(&val)
            },
            None => {}
        };

        match query.get_option() {
            Some(val) => {
                query_string.push_str(" OPTION ");
                query_string.push_str(&val)
            },
            None => {}
        };

        // println!("select query: {0}",&query_string);

        // let array: &[&ToSql];
        // let arr_vec: Vec<&ToSql>;
        // match query.get_bind() {
        //     Some(val) => {
        //         let arr_temp = val.as_slice();
        //         for el in arr_temp {
        //             arr_vec.push(&el);
        //         }

        //         array = arr_vec.as_slice();
        //     },
        //     None => {
        //         array = &[];
        //     }
        // }

        query_string
    }

    fn into_common_row(postgres_rows : Rows)-> QueryResult {
        let size = postgres_rows.columns().len();
        let mut columns = Vec::new();
        for column in postgres_rows.columns() {
            columns.push(column.name().to_string());
        }

        let mut query_res = QueryResult::new(columns);

        for row in postgres_rows.iter() {
            // let mut common_row = Vec::new();
            // for index in 0..size {
            //     let field : ColEnum<String> = ColEnum::SomeValue(row.get(index));
            //     common_row.push(field);
            // }

            let id : ColEnum<i32> = ColEnum::SomeValue(row.get(0));
            let name : ColEnum<String> = ColEnum::SomeValue(row.get(1));

            let id_val = match id {
                ColEnum::SomeValue(val) => val,
                _ => 0
            };

            let name_val = match name {
                ColEnum::SomeValue(val) => val,
                _ => "".to_string()
            };

            // println!("id = {0}, name = {1}", id_val, name_val);
            
            //query_res.insert(common_row);
        }

        query_res
    }

    pub fn init(user: &str, password: &str, database: &str)-> Result<CustomPostgres,Error>{
        let result = CustomPostgres::create_connection(user,password,database);

        match result{
            Ok(conn) => {
                let postgres_obj = CustomPostgres{connection: conn};
                Result::Ok(postgres_obj)
            },
            Err(err) => Result::Err(err)
        }
    }

    pub fn close_connection(self)->Result<(),Error>{
        self.connection.finish()
    }

    pub fn insert<T: ModelInfo>(&self, model : &T) -> Result<Rows,Error>{
        self.connection.query(&CustomPostgres::form_insert_query(model), &[])
    }

    pub fn insert_model<T: ModelInfo>(&self, model : &T)->i32{
        let result = self.connection.query(&CustomPostgres::form_insert_query(model), &[]).expect("Some error in insert model");

        result.get(0).get(model.get_primary_keys_names()[0].as_str())
    }

    pub fn update<T: ModelInfo>(&self, model : &T) -> Result<u64,Error>{
        self.connection.execute(&CustomPostgres::form_update_query(model), &[])
    }

    pub fn delete<T: ModelInfo>(&self, model : T) -> Result<u64,Error>{
        self.connection.execute(&CustomPostgres::form_delete_query(&model), &[])
    }

    pub fn select(&self, query : &CustomQuery) -> Result<Rows, Error>{
        let query_str = CustomPostgres::form_select_query(query);

        let result = self.connection.query(query_str.as_str(), &[]);

        // match result {
        //     Ok(val) => Ok(CustomPostgres::into_common_row(val)),
        //     Err(err) => Err(err)
        // }

        result
    }

    pub fn execute(&self, query : &str) -> Result<Rows, Error>{
        self.connection.query(query, &[])
    }
}