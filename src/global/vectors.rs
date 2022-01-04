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

pub fn concate_with_rating(vec1: &HashMap<String,f32>, vec2: &HashMap<String,f32>, rating: i32)->HashMap<String,f32> {
    let mut sum_vector: HashMap<String,f32> = vec1.clone();

    let factor = ((rating-5) as f32)*0.2;

    println!("factor = {}",&factor.to_string() );

    let mut count_found = 0;
    let mut count_not_found = 0;
    
    for (key,val2) in vec2 {
        match sum_vector.get(key) {
            Some(val1) => {
                count_found+=1;
                sum_vector.insert(key.clone(), val2*factor+val1);
            },
            None => {
                count_not_found+=1;
                sum_vector.insert(key.clone(),*val2*factor);
            }
        }
    }

    println!(
        "Совпало: {}, не совпало: {}",&count_found.to_string(), &count_not_found.to_string()
    );

    sum_vector
}

pub fn concate_and_normalize(vec1: &HashMap<String,f32>, vec2: &HashMap<String,f32>)->HashMap<String,f32>{
    let sum_vector = concate(vec1, vec2);
    normalize(&sum_vector)
}

pub fn normalize(vec: &HashMap<String,f32>)->HashMap<String,f32>{
    let mut vector_normalized = HashMap::new();

    let len = get_vector_len_string(&vec);

    if len == 0.0 {
        return HashMap::new();
    }

    for (key,val) in vec {
        vector_normalized.insert(key.clone(), val/(len as f32));
    }

    vector_normalized
}

pub fn concate_and_normalize_with_rating(vec1: &HashMap<String,f32>, 
    vec2: &HashMap<String,f32>, rating: i32)
    ->HashMap<String,f32>
    {
    let sum_vector = concate_with_rating(vec1, vec2, rating);

    // println!("Вектор до нормализации = {}",utils::(&sum_vector));

    normalize(&sum_vector)
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
            None=>{
            } 
        }
    }

    res
}