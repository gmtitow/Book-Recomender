use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct BooksFiles {
    pub book_id: IntCol,
    pub file_id: IntCol
}
impl BooksFiles {
    pub fn set_book_id(&mut self, book_id : ColEnum<i32>)->&Self{
        self.book_id = IntCol::new_col(book_id);
        self
    }

    pub fn get_book_id(&self)->&ColEnum<i32>{
        &self.book_id.value
    }

    pub fn set_file_id(&mut self, file_id : ColEnum<i32>)->&Self{
        self.file_id = IntCol::new_col(file_id);
        self
    }

    pub fn get_file_id(&self)->&ColEnum<i32>{
        &self.file_id.value
    }
}
impl ModelInfo for BooksFiles {
    fn get_table_name(&self) -> &str{
        "books_files"
    }

    fn get_source() -> String {
        "books_files".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["book_id".to_string(), "file_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.book_id.convert_to_sql(),
            self.file_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["book_id","file_id"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.book_id.convert_to_sql(),
            self.file_id.convert_to_sql()
        ]
    }
}