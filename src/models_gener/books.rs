use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct Books {
    pub book_id: IntCol,
    pub name: StringCol,
    pub genre_id: IntCol,
    pub author_id: IntCol,
    pub date_adding: StringCol,
    pub file_id: IntCol
}
impl Books {
    pub fn set_book_id(&mut self, book_id : ColEnum<i32>)->&Self{
        self.book_id = IntCol::new_col(book_id);
        self
    }

    pub fn get_book_id(&self)->&ColEnum<i32>{
        &self.book_id.value
    }

    pub fn set_name(&mut self, name : ColEnum<String>)->&Self{
        self.name = StringCol::new_col(name);
        self
    }

    pub fn get_name(&self)->&ColEnum<String>{
        &self.name.value
    }

    pub fn set_genre_id(&mut self, genre_id : ColEnum<i32>)->&Self{
        self.genre_id = IntCol::new_col(genre_id);
        self
    }

    pub fn get_genre_id(&self)->&ColEnum<i32>{
        &self.genre_id.value
    }

    pub fn set_author_id(&mut self, author_id : ColEnum<i32>)->&Self{
        self.author_id = IntCol::new_col(author_id);
        self
    }

    pub fn get_author_id(&self)->&ColEnum<i32>{
        &self.author_id.value
    }

    pub fn set_date_adding(&mut self, date_adding : ColEnum<String>)->&Self{
        self.date_adding = StringCol::new_col(date_adding);
        self
    }

    pub fn get_date_adding(&self)->&ColEnum<String>{
        &self.date_adding.value
    }

    pub fn set_file_id(&mut self, file_id : ColEnum<i32>)->&Self{
        self.file_id = IntCol::new_col(file_id);
        self
    }

    pub fn get_file_id(&self)->&ColEnum<i32>{
        &self.file_id.value
    }
}
impl ModelInfo for Books {
    fn get_table_name(&self) -> &str{
        "books"
    }

    fn get_source() -> String {
        "books".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["book_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.book_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["book_id","name","genre_id","author_id","date_adding","file_id"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.book_id.convert_to_sql(),
            self.name.convert_to_sql(),
            self.genre_id.convert_to_sql(),
            self.author_id.convert_to_sql(),
            self.date_adding.convert_to_sql(),
            self.file_id.convert_to_sql()
        ]
    }
}