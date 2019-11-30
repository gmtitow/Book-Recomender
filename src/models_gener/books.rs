use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct Books {
    pub book_id: IntCol,
    pub name: StringCol,
    pub author_id: IntCol,
    pub date_adding: StringCol,
    pub translate: StringCol,
    pub rating_parsed: FloatCol64,
    pub description: StringCol,
    pub publishing_year: IntCol,
    pub reading_id: IntCol,
    pub translator_id: IntCol
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

    pub fn set_translate(&mut self, translate : ColEnum<String>)->&Self{
        self.translate = StringCol::new_col(translate);
        self
    }

    pub fn get_translate(&self)->&ColEnum<String>{
        &self.translate.value
    }

    pub fn set_rating_parsed(&mut self, rating_parsed : ColEnum<f64>)->&Self{
        self.rating_parsed = FloatCol64::new_col(rating_parsed);
        self
    }

    pub fn get_rating_parsed(&self)->&ColEnum<f64>{
        &self.rating_parsed.value
    }

    pub fn set_description(&mut self, description : ColEnum<String>)->&Self{
        self.description = StringCol::new_col(description);
        self
    }

    pub fn get_description(&self)->&ColEnum<String>{
        &self.description.value
    }

    pub fn set_publishing_year(&mut self, publishing_year : ColEnum<i32>)->&Self{
        self.publishing_year = IntCol::new_col(publishing_year);
        self
    }

    pub fn get_publishing_year(&self)->&ColEnum<i32>{
        &self.publishing_year.value
    }

    pub fn set_reading_id(&mut self, reading_id : ColEnum<i32>)->&Self{
        self.reading_id = IntCol::new_col(reading_id);
        self
    }

    pub fn get_reading_id(&self)->&ColEnum<i32>{
        &self.reading_id.value
    }

    pub fn set_translator_id(&mut self, translator_id : ColEnum<i32>)->&Self{
        self.translator_id = IntCol::new_col(translator_id);
        self
    }

    pub fn get_translator_id(&self)->&ColEnum<i32>{
        &self.translator_id.value
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
        vec!["book_id","name","author_id","date_adding","translate","rating_parsed","description","publishing_year","reading_id","translator_id"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.book_id.convert_to_sql(),
            self.name.convert_to_sql(),
            self.author_id.convert_to_sql(),
            self.date_adding.convert_to_sql(),
            self.translate.convert_to_sql(),
            self.rating_parsed.convert_to_sql(),
            self.description.convert_to_sql(),
            self.publishing_year.convert_to_sql(),
            self.reading_id.convert_to_sql(),
            self.translator_id.convert_to_sql()
        ]
    }
}