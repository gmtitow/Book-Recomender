use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

#[derive(Default)]
pub struct VecSums {
    pub id: IntCol,
    pub vec: StringCol,
    pub vec_sum: StringCol
}
impl VecSums {
    pub fn set_id(&mut self, id : ColEnum<i32>)->&Self{
        self.id = IntCol::new_col(id);
        self
    }

    pub fn get_id(&self)->&ColEnum<i32>{
        &self.id.value
    }

    pub fn set_vec(&mut self, vec : ColEnum<String>)->&Self{
        self.vec = StringCol::new_col(vec);
        self
    }

    pub fn get_vec(&self)->&ColEnum<String>{
        &self.vec.value
    }

    pub fn set_vec_sum(&mut self, vec_sum : ColEnum<String>)->&Self{
        self.vec_sum = StringCol::new_col(vec_sum);
        self
    }

    pub fn get_vec_sum(&self)->&ColEnum<String>{
        &self.vec_sum.value
    }
}
impl ModelInfo for VecSums {
    fn get_table_name(&self) -> &str{
        "vec_sums"
    }

    fn get_source() -> String {
        "vec_sums".to_owned()
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
        vec!["id","vec","vec_sum"]
    }

    fn get_values(&self) -> Vec<Option<String>>{
        vec![
            self.id.convert_to_sql(),
            self.vec.convert_to_sql(),
            self.vec_sum.convert_to_sql()
        ]
    }
}