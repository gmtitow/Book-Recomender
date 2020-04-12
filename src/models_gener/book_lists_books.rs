use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct BookListsBooks {
    pub list_id: IntCol,
    pub book_id: IntCol,
    pub created_at: StringCol,
    pub rating: IntCol
}
impl BookListsBooks {
    pub fn set_list_id(&mut self, list_id : ColEnum<i32>)->&Self{
        self.list_id = IntCol::new_col(list_id);
        self
    }

    pub fn get_list_id(&self)->&ColEnum<i32>{
        &self.list_id.value
    }

    pub fn set_book_id(&mut self, book_id : ColEnum<i32>)->&Self{
        self.book_id = IntCol::new_col(book_id);
        self
    }

    pub fn get_book_id(&self)->&ColEnum<i32>{
        &self.book_id.value
    }

    pub fn set_created_at(&mut self, created_at : ColEnum<String>)->&Self{
        self.created_at = StringCol::new_col(created_at);
        self
    }

    pub fn get_created_at(&self)->&ColEnum<String>{
        &self.created_at.value
    }

    pub fn set_rating(&mut self, rating : ColEnum<i32>)->&Self{
        self.rating = IntCol::new_col(rating);
        self
    }

    pub fn get_rating(&self)->&ColEnum<i32>{
        &self.rating.value
    }
}
impl ModelInfo for BookListsBooks {
    fn get_table_name(&self) -> &str{
        "book_lists_books"
    }

    fn get_source() -> String {
        "book_lists_books".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["list_id".to_string(), "book_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.list_id.convert_to_sql(),
            self.book_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["list_id","book_id","created_at","rating"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.list_id.convert_to_sql(),
            self.book_id.convert_to_sql(),
            self.created_at.convert_to_sql(),
            self.rating.convert_to_sql()
        ]
    }
}