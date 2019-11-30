use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct Authors {
    pub author_id: IntCol,
    pub first_name: StringCol,
    pub last_name: StringCol,
    pub full_name: StringCol,
    pub birthday: StringCol,
    pub date_adding: StringCol,
    pub reading_id: IntCol,
    pub vector: StringCol
}
impl Authors {
    pub fn set_author_id(&mut self, author_id : ColEnum<i32>)->&Self{
        self.author_id = IntCol::new_col(author_id);
        self
    }

    pub fn get_author_id(&self)->&ColEnum<i32>{
        &self.author_id.value
    }

    pub fn set_first_name(&mut self, first_name : ColEnum<String>)->&Self{
        self.first_name = StringCol::new_col(first_name);
        self
    }

    pub fn get_first_name(&self)->&ColEnum<String>{
        &self.first_name.value
    }

    pub fn set_last_name(&mut self, last_name : ColEnum<String>)->&Self{
        self.last_name = StringCol::new_col(last_name);
        self
    }

    pub fn get_last_name(&self)->&ColEnum<String>{
        &self.last_name.value
    }

    pub fn set_full_name(&mut self, full_name : ColEnum<String>)->&Self{
        self.full_name = StringCol::new_col(full_name);
        self
    }

    pub fn get_full_name(&self)->&ColEnum<String>{
        &self.full_name.value
    }

    pub fn set_birthday(&mut self, birthday : ColEnum<String>)->&Self{
        self.birthday = StringCol::new_col(birthday);
        self
    }

    pub fn get_birthday(&self)->&ColEnum<String>{
        &self.birthday.value
    }

    pub fn set_date_adding(&mut self, date_adding : ColEnum<String>)->&Self{
        self.date_adding = StringCol::new_col(date_adding);
        self
    }

    pub fn get_date_adding(&self)->&ColEnum<String>{
        &self.date_adding.value
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
impl ModelInfo for Authors {
    fn get_table_name(&self) -> &str{
        "authors"
    }

    fn get_source() -> String {
        "authors".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["author_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.author_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["author_id","first_name","last_name","full_name","birthday","date_adding","reading_id","vector"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.author_id.convert_to_sql(),
            self.first_name.convert_to_sql(),
            self.last_name.convert_to_sql(),
            self.full_name.convert_to_sql(),
            self.birthday.convert_to_sql(),
            self.date_adding.convert_to_sql(),
            self.reading_id.convert_to_sql(),
            self.vector.convert_to_sql()
        ]
    }
}