# Hermes

A very convenient string parsing crate that can parse function and variable references in strings into corresponding values.

## String Syntax Example
```rust
let input = "${hostname()}+\"dd\"+true+${name}+${invalid}+${random_str(${len})}+${multiply(1,-2.5,3)}";
```

The string can contain function calls, variable references, booleans, numbers, and strings, connected by "+".

+ `Function Expression`: syntax is `${function_name(args...)}`, where function_name is the name of a function, and args are multiple parameters, which can also be absent, depending on the parameters required by the function. Args can also be function calls, variable references, booleans, numbers, and strings.
+ `Variable Expression`: syntax is `${variable_name}`.
+ `Boolean Expression`: just `true` or `false`.
+ `Number Expression`: like `10`, `-12`, `12.56` and so on.
+ `String Expression`: any characters between two double quotation marks. like `"foo and bar"`.

## How to use

You need to initialize a `Cache` struct to store all your variables and functions. Then, you can call `parse` or `parse_to_string` function to parse the a string.

See example below:
```rust
use hermes_ru::{cache::{ ExpType, Cache, CacheVariable }, error::Error};

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
    println!("parse result is: {}", hermes::lexer::parse_to_string(input, &mut cache));

}
```

Output:
```bash
parse result is: MacBook-Pro.localddtrueliudaojdTXX8f16d-7.5
```

## Inner Function

+ `random_str(len)` Get a random string with length.
+ `random_bool()` Get a random boolean value.
+ `random_num()` or `random_num(end)` or `random_num(start,end)` Get a random number.
+ `hostname()` Get your hostname.
+ `current_time` Get current time with format.

