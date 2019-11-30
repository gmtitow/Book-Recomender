use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct Genres {
    pub genre_id: IntCol,
    pub genre_name: StringCol,
    pub genre_name_english: StringCol,
    pub reading_id: IntCol,
    pub vector: StringCol
}
impl Genres {
    pub fn set_genre_id(&mut self, genre_id : ColEnum<i32>)->&Self{
        self.genre_id = IntCol::new_col(genre_id);
        self
    }

    pub fn get_genre_id(&self)->&ColEnum<i32>{
        &self.genre_id.value
    }

    pub fn set_genre_name(&mut self, genre_name : ColEnum<String>)->&Self{
        self.genre_name = StringCol::new_col(genre_name);
        self
    }

    pub fn get_genre_name(&self)->&ColEnum<String>{
        &self.genre_name.value
    }

    pub fn set_genre_name_english(&mut self, genre_name_english : ColEnum<String>)->&Self{
        self.genre_name_english = StringCol::new_col(genre_name_english);
        self
    }

    pub fn get_genre_name_english(&self)->&ColEnum<String>{
        &self.genre_name_english.value
    }

    pub fn set_reading_id(&mut self, reading_id : ColEnum<i32>)->&Self{
        self.reading_id = IntCol::new_col(reading_id);
        self
    }

    pub fn get_reading_id(&self)->&ColEnum<i32>{
        &self.reading_id.value
    }

    pub fn set_vector(&mut self, vector : ColEnum<String>)->&Self{
        self.vector = StringCol::new_col(vector);
        self
    }

    pub fn get_vector(&self)->&ColEnum<String>{
        &self.vector.value
    }
}
impl ModelInfo for Genres {
    fn get_table_name(&self) -> &str{
        "genres"
    }

    fn get_source() -> String {
        "genres".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["genre_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.genre_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["genre_id","genre_name","genre_name_english","reading_id","vector"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.genre_id.convert_to_sql(),
            self.genre_name.convert_to_sql(),
            self.genre_name_english.convert_to_sql(),
            self.reading_id.convert_to_sql(),
            self.vector.convert_to_sql()
        ]
    }
}