use hermes_ru::{cache::{ ExpType, Cache, CacheVariable }, error::Error, lexer::parse_to_string};

fn main() {
    // new a cache with 10 size capacity
    let mut cache = Cache::new(10);
    // add two variables to cache
    cache.add_variable(CacheVariable::new("name".to_string(), ExpType::STRING("liudao".to_string())));
    cache.add_variable(CacheVariable::new("len".to_string(), ExpType::INT(10)));

    // add a custom function to cache
    cache.add_function("multiply", Box::new(|params: Vec<ExpType>| -> Result<ExpType, Error>{
        Ok(params.into_iter().fold(ExpType::FLOAT(1.0f32), |acc, item| {
            let acc_num = if let ExpType::FLOAT(float) = acc {
                float
            } else {
                0.0f32
            };
            match item {
                ExpType::INT(i) => ExpType::FLOAT(acc_num * i as f32),
                ExpType::FLOAT(f) => ExpType::FLOAT(acc_num * f),
                _ => acc,
            }
        }))
        
    }));

    // define a complex str
    let input = "${hostname()}+\"dd\"+true+${name}+${invalid}+${random_str(${len})}+${multiply(1,-2.5,3)}}";
    
    // parse the str
    println!("parse result is: {}", parse_to_string(input, &mut cache));

}
