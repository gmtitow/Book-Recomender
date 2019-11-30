use std::collections::HashMap;

pub fn concate(vec1: &HashMap<String,f32>, vec2: &HashMap<String,f32>)->HashMap<String,f32> {
    let mut sum_vector: HashMap<String,f32> = vec1.clone();

    for (key,val2) in vec2 {
        match sum_vector.get(key) {
            Some(val1) => {
                sum_vector.insert(key.clone(), val2+val1);
            },
            None => {
                sum_vector.insert(key.clone(),*val2);
            }
        }
    }

    sum_vector
}

pub fn concate_and_normalize(vec1: &HashMap<String,f32>, vec2: &HashMap<String,f32>)->HashMap<String,f32>{
    let sum_vector = concate(vec1, vec2);
    let mut sum_vector_normalized = HashMap::new();

    let len = get_vector_len_string(&sum_vector);

    for (key,val) in sum_vector {
        sum_vector_normalized.insert(key.clone(), val/(len as f32));
    }

    sum_vector_normalized
}

pub fn get_vector_len_string(vector: &HashMap<String, f32>)->f64 {
    let mut sum: f64 = 0.0;
    for (_,value) in vector {
        let val: f64 = (*value).into();
        sum = sum + val*val;
    }

    sum.sqrt()
}

pub fn get_cos_normal(vec1: &HashMap<String,f32>, vec2: &HashMap<String,f32>)->f32{
    let mut res: f32 = 0.0;

    for (key,val1) in vec1 {
        match vec2.get(key) {
            Some(val2)=>{
                res = res + val1*val2;
            },
            None=>{} 
        }
    }

    res
}