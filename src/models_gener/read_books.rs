use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct ReadBooks {
    pub user_id: IntCol,
    pub book_id: IntCol,
    pub date_adding: StringCol,
    pub rating: IntCol
}
impl ReadBooks {
    pub fn set_user_id(&mut self, user_id : ColEnum<i32>)->&Self{
        self.user_id = IntCol::new_col(user_id);
        self
    }

    pub fn get_user_id(&self)->&ColEnum<i32>{
        &self.user_id.value
    }

    pub fn set_book_id(&mut self, book_id : ColEnum<i32>)->&Self{
        self.book_id = IntCol::new_col(book_id);
        self
    }

    pub fn get_book_id(&self)->&ColEnum<i32>{
        &self.book_id.value
    }

    pub fn set_date_adding(&mut self, date_adding : ColEnum<String>)->&Self{
        self.date_adding = StringCol::new_col(date_adding);
        self
    }

    pub fn get_date_adding(&self)->&ColEnum<String>{
        &self.date_adding.value
    }

    pub fn set_rating(&mut self, rating : ColEnum<i32>)->&Self{
        self.rating = IntCol::new_col(rating);
        self
    }

    pub fn get_rating(&self)->&ColEnum<i32>{
        &self.rating.value
    }
}
impl ModelInfo for ReadBooks {
    fn get_table_name(&self) -> &str{
        "read_books"
    }

    fn get_source() -> String {
        "read_books".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["user_id".to_string(), "book_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.user_id.convert_to_sql(),
            self.book_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["user_id","book_id","date_adding","rating"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.user_id.convert_to_sql(),
            self.book_id.convert_to_sql(),
            self.date_adding.convert_to_sql(),
            self.rating.convert_to_sql()
        ]
    }
}