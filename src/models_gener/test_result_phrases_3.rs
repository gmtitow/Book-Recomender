use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct TestResultPhrases3 {
    pub id: IntCol,
    pub genre: StringCol,
    pub stats_genre: StringCol,
    pub count: IntCol
}
impl TestResultPhrases3 {
    pub fn set_id(&mut self, id : ColEnum<i32>)->&Self{
        self.id = IntCol::new_col(id);
        self
    }

    pub fn get_id(&self)->&ColEnum<i32>{
        &self.id.value
    }

    pub fn set_genre(&mut self, genre : ColEnum<String>)->&Self{
        self.genre = StringCol::new_col(genre);
        self
    }

    pub fn get_genre(&self)->&ColEnum<String>{
        &self.genre.value
    }

    pub fn set_stats_genre(&mut self, stats_genre : ColEnum<String>)->&Self{
        self.stats_genre = StringCol::new_col(stats_genre);
        self
    }

    pub fn get_stats_genre(&self)->&ColEnum<String>{
        &self.stats_genre.value
    }

    pub fn set_count(&mut self, count : ColEnum<i32>)->&Self{
        self.count = IntCol::new_col(count);
        self
    }

    pub fn get_count(&self)->&ColEnum<i32>{
        &self.count.value
    }
}
impl ModelInfo for TestResultPhrases3 {
    fn get_table_name(&self) -> &str{
        "test_result_phrases_3"
    }

    fn get_source() -> String {
        "test_result_phrases_3".to_owned()
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
        vec!["id","genre","stats_genre","count"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.id.convert_to_sql(),
            self.genre.convert_to_sql(),
            self.stats_genre.convert_to_sql(),
            self.count.convert_to_sql()
        ]
    }
}