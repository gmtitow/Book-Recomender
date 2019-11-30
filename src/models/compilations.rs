use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct Compilations {
    pub compilation_id: IntCol,
    pub name: StringCol,
    pub reading_id: IntCol
}
impl Compilations {
    pub fn set_compilation_id(&mut self, compilation_id : ColEnum<i32>)->&Self{
        self.compilation_id = IntCol::new_col(compilation_id);
        self
    }

    pub fn get_compilation_id(&self)->&ColEnum<i32>{
        &self.compilation_id.value
    }

    pub fn set_name(&mut self, name : ColEnum<String>)->&Self{
        self.name = StringCol::new_col(name);
        self
    }

    pub fn get_name(&self)->&ColEnum<String>{
        &self.name.value
    }

    pub fn set_reading_id(&mut self, reading_id : ColEnum<i32>)->&Self{
        self.reading_id = IntCol::new_col(reading_id);
        self
    }

    pub fn get_reading_id(&self)->&ColEnum<i32>{
        &self.reading_id.value
    }
}
impl ModelInfo for Compilations {
    fn get_table_name(&self) -> &str{
        "compilations"
    }

    fn get_source() -> String {
        "compilations".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["compilation_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.compilation_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["compilation_id","name","reading_id"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.compilation_id.convert_to_sql(),
            self.name.convert_to_sql(),
            self.reading_id.convert_to_sql()
        ]
    }
}