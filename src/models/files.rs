use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct Files {
    pub file_id: IntCol,
    pub full_name: StringCol,
    pub created_at: StringCol,
    pub extension: StringCol,
    pub name: StringCol,
    pub path_to: StringCol
}
impl Files {
    pub fn set_file_id(&mut self, file_id : ColEnum<i32>)->&Self{
        self.file_id = IntCol::new_col(file_id);
        self
    }

    pub fn get_file_id(&self)->&ColEnum<i32>{
        &self.file_id.value
    }

    pub fn set_full_name(&mut self, full_name : ColEnum<String>)->&Self{
        self.full_name = StringCol::new_col(full_name);
        self
    }

    pub fn get_full_name(&self)->&ColEnum<String>{
        &self.full_name.value
    }

    pub fn set_created_at(&mut self, created_at : ColEnum<String>)->&Self{
        self.created_at = StringCol::new_col(created_at);
        self
    }

    pub fn get_created_at(&self)->&ColEnum<String>{
        &self.created_at.value
    }

    pub fn set_extension(&mut self, extension : ColEnum<String>)->&Self{
        self.extension = StringCol::new_col(extension);
        self
    }

    pub fn get_extension(&self)->&ColEnum<String>{
        &self.extension.value
    }

    pub fn set_name(&mut self, name : ColEnum<String>)->&Self{
        self.name = StringCol::new_col(name);
        self
    }

    pub fn get_name(&self)->&ColEnum<String>{
        &self.name.value
    }

    pub fn set_path_to(&mut self, path_to : ColEnum<String>)->&Self{
        self.path_to = StringCol::new_col(path_to);
        self
    }

    pub fn get_path_to(&self)->&ColEnum<String>{
        &self.path_to.value
    }
}
impl ModelInfo for Files {
    fn get_table_name(&self) -> &str{
        "files"
    }

    fn get_source() -> String {
        "files".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["file_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.file_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["file_id","full_name","created_at","extension","name","path_to"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.file_id.convert_to_sql(),
            self.full_name.convert_to_sql(),
            self.created_at.convert_to_sql(),
            self.extension.convert_to_sql(),
            self.name.convert_to_sql(),
            self.path_to.convert_to_sql()
        ]
    }
}