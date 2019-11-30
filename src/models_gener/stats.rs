use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct Stats {
    pub id: IntCol,
    pub description: StringCol,
    pub data: StringCol
}
impl Stats {
    pub fn set_id(&mut self, id : ColEnum<i32>)->&Self{
        self.id = IntCol::new_col(id);
        self
    }

    pub fn get_id(&self)->&ColEnum<i32>{
        &self.id.value
    }

    pub fn set_description(&mut self, description : ColEnum<String>)->&Self{
        self.description = StringCol::new_col(description);
        self
    }

    pub fn get_description(&self)->&ColEnum<String>{
        &self.description.value
    }

    pub fn set_data(&mut self, data : ColEnum<String>)->&Self{
        self.data = StringCol::new_col(data);
        self
    }

    pub fn get_data(&self)->&ColEnum<String>{
        &self.data.value
    }
}
impl ModelInfo for Stats {
    fn get_table_name(&self) -> &str{
        "stats"
    }

    fn get_source() -> String {
        "stats".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["id","description","data"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.id.convert_to_sql(),
            self.description.convert_to_sql(),
            self.data.convert_to_sql()
        ]
    }
}