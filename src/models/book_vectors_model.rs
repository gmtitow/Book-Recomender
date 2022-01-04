use crate::database::common::{ModelInfo,IntCol,IntCol64,FloatCol64,StringCol,ColEnum,SqlValue};

pub trait BookVectorsModel : ModelInfo{
    

    fn get_book_id(&self)->&ColEnum<i32>;

    fn get_vector_direct(&self)->&ColEnum<String>;

    fn get_vector_hashed(&self)->&ColEnum<String>;

    fn get_vector_invers(&self)->&ColEnum<String>;

    fn get_length(&self)->&ColEnum<f64>;

    fn set_length(&mut self, length : ColEnum<f64>)->&Self;
    fn set_book_id(&mut self, length : ColEnum<i32>)->&Self;
    fn set_vector_direct(&mut self, length : ColEnum<String>)->&Self;
    fn set_vector_invers(&mut self, length : ColEnum<String>)->&Self;
    fn set_vector_hashed(&mut self, length : ColEnum<String>)->&Self;
}