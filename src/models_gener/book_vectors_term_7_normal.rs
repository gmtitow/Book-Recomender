use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct BookVectorsTerm7Normal {
    pub book_id: IntCol,
    pub vector_direct: StringCol,
    pub vector_invers: StringCol,
    pub length: FloatCol64
}
impl BookVectorsTerm7Normal {
    pub fn set_book_id(&mut self, book_id : ColEnum<i32>)->&Self{
        self.book_id = IntCol::new_col(book_id);
        self
    }

    pub fn get_book_id(&self)->&ColEnum<i32>{
        &self.book_id.value
    }

    pub fn set_vector_direct(&mut self, vector_direct : ColEnum<String>)->&Self{
        self.vector_direct = StringCol::new_col(vector_direct);
        self
    }

    pub fn get_vector_direct(&self)->&ColEnum<String>{
        &self.vector_direct.value
    }

    pub fn set_vector_invers(&mut self, vector_invers : ColEnum<String>)->&Self{
        self.vector_invers = StringCol::new_col(vector_invers);
        self
    }

    pub fn get_vector_invers(&self)->&ColEnum<String>{
        &self.vector_invers.value
    }

    pub fn set_length(&mut self, length : ColEnum<f64>)->&Self{
        self.length = FloatCol64::new_col(length);
        self
    }

    pub fn get_length(&self)->&ColEnum<f64>{
        &self.length.value
    }
}
impl ModelInfo for BookVectorsTerm7Normal {
    fn get_table_name(&self) -> &str{
        "book_vectors_term_7_normal"
    }

    fn get_source() -> String {
        "book_vectors_term_7_normal".to_owned()
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
        vec!["book_id","vector_direct","vector_invers","length"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.book_id.convert_to_sql(),
            self.vector_direct.convert_to_sql(),
            self.vector_invers.convert_to_sql(),
            self.length.convert_to_sql()
        ]
    }
}