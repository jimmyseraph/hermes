use crate::{cache::ExpType, error::Error};

/// Get a random string with length
/// 
/// # Example
/// ```rust
/// use hermes::{cache::ExpType, func::random_str};
/// let params: Vec<ExpType> = vec![ExpType::INT(10)];
/// let str = random_str(params).unwrap();
/// assert_eq!(str.to_string().len(), 10);
/// ```
pub fn random_str(params: Vec<ExpType>) -> Result<ExpType, Error> {
    use random_string::generate;
    if params.len() != 1 {
        return Err(Error::new("random_str function only accept one parameter".to_string()));
    }
    let length = match params[0] {
        ExpType::INT(i) => i,
        _ => return Err(Error::new("random_str function only accept one int32 parameter".to_string())),
    };
    let charset = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    Ok(ExpType::STRING(generate(length as usize, charset)))
}

/// Get a random boolean value
/// 
/// # Example
/// ```rust
/// use hermes::{cache::ExpType, func::random_bool};
/// let params: Vec<ExpType> = vec![];
/// let bool = random_bool(params).unwrap();
/// assert_eq!(bool.to_string().parse::<bool>().is_ok(), true);
/// ```
pub fn random_bool(_params: Vec<ExpType>) -> Result<ExpType, Error> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    Ok(ExpType::BOOL(rng.gen::<bool>()))
}

/// Get a random number
/// 
/// If no parameter is passed, the random number is a int32.
/// # Example
/// ```rust
/// use hermes::{cache::ExpType, func::random_num};
/// let params: Vec<ExpType> = vec![];
/// let num = random_num(params).unwrap();
/// assert_eq!(num.to_string().parse::<i32>().is_ok(), true);
/// ```
/// 
/// If one parameter is passed, the random number is a int32 or float32.
/// # Example
/// ```rust
/// use hermes::{cache::ExpType, func::random_num};
/// let params: Vec<ExpType> = vec![ExpType::FLOAT(10.0f32)];
/// let num = random_num(params).unwrap();
/// assert_eq!(num.to_string().parse::<f32>().unwrap() < 10.0f32, true);
/// ```
/// 
/// If two parameters are passed, the random number is a int32 or float32 between the two parameters.
/// # Example
/// ```rust
/// use hermes::{cache::ExpType, func::random_num};
/// let params: Vec<ExpType> = vec![ExpType::INT(10), ExpType::INT(20)];
/// let num = random_num(params).unwrap();
/// assert_eq!(num.to_string().parse::<i32>().unwrap() < 20 && num.to_string().parse::<i32>().unwrap() >= 10, true);
/// ```
pub fn random_num(params: Vec<ExpType>) -> Result<ExpType, Error> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    match params.len() {
        0 => {
            Ok(ExpType::INT(rng.gen::<i32>()))
        },
        1 => {
            match params[0] {
                ExpType::INT(i) => Ok(ExpType::INT(rng.gen_range(0..i))),
                ExpType::FLOAT(f) => Ok(ExpType::FLOAT(rng.gen_range(0.0f32..f))),
                _ => Err(Error::new("random_num function only accept a int32 or float32 parameter".to_string())),
            }
        },
        2 => {
            match (params[0].clone(), params[1].clone()) {
                (ExpType::INT(i1), ExpType::INT(i2)) => Ok(ExpType::INT(rng.gen_range(i1..i2))),
                (ExpType::FLOAT(f1), ExpType::FLOAT(f2)) => Ok(ExpType::FLOAT(rng.gen_range(f1..f2))),
                _ => Err(Error::new("random_num function only accept two int32 or float32 parameter".to_string())),
            }
        }
        _ => Err(Error::new("random_num function only accept 0-2 parameters".to_string())),
    }
}

pub fn hostname(_params: Vec<ExpType>) -> Result<ExpType, Error> {
    hostname::get()
    .map(|s| ExpType::STRING(s.into_string().unwrap()))
    .map_err(|e| Error::new(e.to_string()))
}

/// Get current time with format
/// See the [`crate::format::strftime`] module for the supported escape sequences.
///
/// # Example
/// ```rust
/// use hermes::{cache::ExpType, error::Error, func::current_time};
/// let params = vec![ExpType::STRING("%d/%m/%Y %H:%M".to_string())];
/// let current_time_str = current_time(params).unwrap().to_string();
/// assert_eq!(current_time_str.len(), "02/04/2023 12:50".len());
/// ```
pub fn current_time(params: Vec<ExpType>) -> Result<ExpType, Error> {
    if params.len() != 1 {
        return Err(Error::new("current_time function only accept one parameter".to_string()));
    }
    let format = match params[0].clone() {
        ExpType::STRING(s) => s,
        _ => return Err(Error::new("current_time function only accept one string parameter".to_string())),
    };
    let now = chrono::Local::now();
    Ok(ExpType::STRING(now.format(&format).to_string()))
}