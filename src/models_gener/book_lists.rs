use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct BookLists {
    pub list_id: IntCol,
    pub list_name: StringCol,
    pub user_id: IntCol,
    pub created_at: StringCol,
    pub is_main: StringCol
}
impl BookLists {
    pub fn set_list_id(&mut self, list_id : ColEnum<i32>)->&Self{
        self.list_id = IntCol::new_col(list_id);
        self
    }

    pub fn get_list_id(&self)->&ColEnum<i32>{
        &self.list_id.value
    }

    pub fn set_list_name(&mut self, list_name : ColEnum<String>)->&Self{
        self.list_name = StringCol::new_col(list_name);
        self
    }

    pub fn get_list_name(&self)->&ColEnum<String>{
        &self.list_name.value
    }

    pub fn set_user_id(&mut self, user_id : ColEnum<i32>)->&Self{
        self.user_id = IntCol::new_col(user_id);
        self
    }

    pub fn get_user_id(&self)->&ColEnum<i32>{
        &self.user_id.value
    }

    pub fn set_created_at(&mut self, created_at : ColEnum<String>)->&Self{
        self.created_at = StringCol::new_col(created_at);
        self
    }

    pub fn get_created_at(&self)->&ColEnum<String>{
        &self.created_at.value
    }

    pub fn set_is_main(&mut self, is_main : ColEnum<String>)->&Self{
        self.is_main = StringCol::new_col(is_main);
        self
    }

    pub fn get_is_main(&self)->&ColEnum<String>{
        &self.is_main.value
    }
}
impl ModelInfo for BookLists {
    fn get_table_name(&self) -> &str{
        "book_lists"
    }

    fn get_source() -> String {
        "book_lists".to_owned()
    }

    fn get_primary_keys_names(&self) -> Vec<String>{
        vec!["list_id".to_string()]
    }

    fn get_primary_keys_values(&self) -> Vec<Option<String>>{
        vec![
            self.list_id.convert_to_sql()
        ]
    }

    fn get_columns(&self)-> Vec<&str>{
        vec!["list_id","list_name","user_id","created_at","is_main"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.list_id.convert_to_sql(),
            self.list_name.convert_to_sql(),
            self.user_id.convert_to_sql(),
            self.created_at.convert_to_sql(),
            self.is_main.convert_to_sql()
        ]
    }
}