use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct GenresBooks {
    pub genre_id: IntCol,
    pub book_id: IntCol
}
impl GenresBooks {
    pub fn set_genre_id(&mut self, genre_id : ColEnum<i32>)->&Self{
        self.genre_id = IntCol::new_col(genre_id);
        self
    }

    pub fn get_genre_id(&self)->&ColEnum<i32>{
        &self.genre_id.value
    }

    pub fn set_book_id(&mut self, book_id : ColEnum<i32>)->&Self{
        self.book_id = IntCol::new_col(book_id);
        self
    }

    pub fn get_book_id(&self)->&ColEnum<i32>{
        &self.book_id.value
    }
}
impl ModelInfo for GenresBooks {
    fn get_table_name(&self) -> &str{
        "genres_books"
    }

    fn get_source() -> String {
        "genres_books".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["genre_id".to_string(), "book_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.genre_id.convert_to_sql(),
            self.book_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["genre_id","book_id"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.genre_id.convert_to_sql(),
            self.book_id.convert_to_sql()
        ]
    }
}