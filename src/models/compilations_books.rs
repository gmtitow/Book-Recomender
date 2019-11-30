use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct CompilationsBooks {
    pub compilation_id: IntCol,
    pub book_id: IntCol
}
impl CompilationsBooks {
    pub fn set_compilation_id(&mut self, compilation_id : ColEnum<i32>)->&Self{
        self.compilation_id = IntCol::new_col(compilation_id);
        self
    }

    pub fn get_compilation_id(&self)->&ColEnum<i32>{
        &self.compilation_id.value
    }

    pub fn set_book_id(&mut self, book_id : ColEnum<i32>)->&Self{
        self.book_id = IntCol::new_col(book_id);
        self
    }

    pub fn get_book_id(&self)->&ColEnum<i32>{
        &self.book_id.value
    }
}
impl ModelInfo for CompilationsBooks {
    fn get_table_name(&self) -> &str{
        "compilations_books"
    }

    fn get_source() -> String {
        "compilations_books".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["compilation_id".to_string(), "book_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.compilation_id.convert_to_sql(),
            self.book_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["compilation_id","book_id"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.compilation_id.convert_to_sql(),
            self.book_id.convert_to_sql()
        ]
    }
}