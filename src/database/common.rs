extern crate regex;

use std::default::Default;
use regex::Regex;


pub trait ModelInfo : Sized{
    fn get_table_name(&self) -> &str;

    fn get_source() -> String;

    fn get_columns(&self) -> Vec<&str>;

    fn get_values(&self) -> Vec<Option<String>>;

    fn get_primary_keys_names(&self) -> Vec<String>;

    fn get_primary_keys_values(&self) -> Vec<Option<String>>;
}

pub trait SqlValue : Sized{
    fn convert_to_sql(&self)->Option<String>;
}

pub enum ColEnum<T>{
    SomeValue(T),
    Nothing,
    Null
}

impl<T> ColEnum<T> {
    pub fn unwrap(self)->T {
        match self{
            ColEnum::SomeValue(val) => val,
            _ => panic!("Column value is null or empty")
        }
    }

    pub fn unwrap_ref(&self)->&T {
        match self{
            ColEnum::SomeValue(val) => val,
            _ => panic!("Column value is null or empty")
        }
    }
}

pub struct StringCol{
    pub value: ColEnum<String>
}

impl SqlValue for StringCol {
    fn convert_to_sql(&self)->Option<String>{
        match &self.value {
            ColEnum::SomeValue(val) => {
                let re_1 = Regex::new("\'").unwrap();

                let replaced = &re_1.replace_all(&val,"");

                // let re_2 = Regex::new("\"").unwrap();

                // let replaced_2 = &re_2.replace_all(&replaced_1,"\\\"");
                // println!("before replace: {0}",&val);
                // let new_val = re.replace_all(val,"\\\'");
                // println!("after replace: {0}",new_val);
                Some("\'".to_string() + &replaced +"\'")
            },
            ColEnum::Nothing => None,
            ColEnum::Null => Some("NULL".to_string())
        }
    }
}

impl StringCol{
    pub fn new(val: &str)->StringCol{
        StringCol {value:ColEnum::SomeValue(val.to_string())}
    }

    pub fn new_col(val: ColEnum<String>)->StringCol{
        StringCol {value:val}
    }
}

impl Default for StringCol{
    fn default()->StringCol{
        StringCol{
            value:ColEnum::Nothing
        }
    }
}

pub struct IntCol{
    pub value: ColEnum<i32>
}

impl SqlValue for IntCol {
    fn convert_to_sql(&self)->Option<String>{
        match &self.value {
            ColEnum::SomeValue(val) => Some(val.to_string()),
            ColEnum::Nothing => None,
            ColEnum::Null => Some("NULL".to_string())
        }
    }
}

impl IntCol{
    pub fn new(val: i32)->IntCol{
        IntCol {value:ColEnum::SomeValue(val)}
    }

    pub fn new_col(val: ColEnum<i32>)->IntCol{
        IntCol {value:val}
    }
}

impl Default for IntCol{
    fn default()->IntCol{
        IntCol{
            value:ColEnum::Nothing
        }
    }
}

pub struct IntCol64{
    pub value: ColEnum<i64>
}

impl SqlValue for IntCol64 {
    fn convert_to_sql(&self)->Option<String>{
        match &self.value {
            ColEnum::SomeValue(val) => Some(val.to_string()),
            ColEnum::Nothing => None,
            ColEnum::Null => Some("NULL".to_string())
        }
    }
}

impl IntCol64{
    pub fn new(val: i64)->IntCol64{
        IntCol64 {value:ColEnum::SomeValue(val)}
    }

    pub fn new_col(val: ColEnum<i64>)->IntCol64{
        IntCol64 {value:val}
    }
}

impl Default for IntCol64{
    fn default()->IntCol64{
        IntCol64{
            value:ColEnum::Nothing
        }
    }
}

pub struct FloatCol64{
    pub value: ColEnum<f64>
}

impl SqlValue for FloatCol64 {
    fn convert_to_sql(&self)->Option<String>{
        match &self.value {
            ColEnum::SomeValue(val) => Some(val.to_string()),
            ColEnum::Nothing => None,
            ColEnum::Null => Some("NULL".to_string())
        }
    }
}

impl FloatCol64{
    pub fn new(val: f64)->FloatCol64{
        FloatCol64 {value:ColEnum::SomeValue(val)}
    }

    pub fn new_col(val: ColEnum<f64>)->FloatCol64{
        FloatCol64 {value:val}
    }
}

impl Default for FloatCol64{
    fn default()->FloatCol64{
        FloatCol64{
            value:ColEnum::Nothing
        }
    }
}

pub struct QueryResult {
    columns : Vec<String>,
    data : Vec<Vec<ColEnum<String>>>
}

impl QueryResult{

    fn error(code: u8) {
        match code {
            0 => {
                panic!("Количество полей в строке {0} не равно количеству столбцов");
            },
            _ => {
                panic!("Неизвестная ошибка");
            }
        }
    }

    pub fn new(columns : Vec<String>) -> QueryResult{
        QueryResult {
            columns : columns,
            data : Vec::new()
        }
    }

    pub fn new_fill(columns : Vec<String>, data: Vec<Vec<ColEnum<String>>>) -> QueryResult{
        for row in &data {
            if row.len() != columns.len() {
                QueryResult::error(0);
            }
        }

        QueryResult {
            columns : columns,
            data : data
        }
    }

    pub fn insert(&mut self, row : Vec<ColEnum<String>>) -> &Self{
        if row.len() != self.columns.len(){
            QueryResult::error(0);
        }

        self.data.push(row);

        self
    }
    
    pub fn columns(&self) -> &Vec<String> {
        &self.columns
    }

    pub fn rows(&self) -> &Vec<Vec<ColEnum<String>>> {
        &self.data
    }
}