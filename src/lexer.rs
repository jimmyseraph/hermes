
use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;
use crate::{cache::{ExpType, Cache}, error::Error};

#[derive(Parser)]
#[grammar = "expression.pest"]
struct ExpParser;

pub fn parser(input: &str, cache: &mut Cache) -> Result<Vec<Result<ExpType, Error>>, Error> {
    let items = ExpParser::parse(Rule::expression, input);
    if let Err(e) = items {
        return Err(Error::from_pest_error(e));
    }
    let items = items.unwrap();
    // println!("{:?}", items);
    let results: Vec<Result<ExpType, Error>> = items.clone().map(|item| {
        parse_value(item, cache)
    }).collect();
    Ok(results)
}

pub fn parse_to_string(input: &str, cache: &mut Cache) -> String {
    let results = parser(input, cache).unwrap();
    results.into_iter().map(|item| {
        match item {
            Ok(v) => v.to_string(),
            Err(_) => "".to_string(),
        }
    }).collect::<Vec<String>>().join("")
}

fn parse_value(pair: Pair<'_, Rule>, cache: &mut Cache) -> Result<ExpType, Error>{
    match pair.as_rule() {
        Rule::variable => {
            let opt = cache.get_variable(pair.into_inner().next().unwrap().as_str());
            if let Some(v) = opt {
                Ok(v.value.clone())
            } else {
                Err(Error::new("variable not found".to_string()))
            }
        },
        Rule::function => {
            parse_function(pair, cache)
        },
        Rule::number => {
            let number_str = pair.as_str();
            if let true = number_str.contains(".")  {
                number_str.parse::<f32>()
                .map(|num| ExpType::FLOAT(num))
                .map_err(|e| Error::new(e.to_string()))
            } else {
                number_str.parse::<i32>()
                .map(|num| ExpType::INT(num))
                .map_err(|e| Error::new(e.to_string()))
            }
        },
        Rule::boolean => pair.as_str().parse::<bool>().map(|v| ExpType::BOOL(v)).map_err(|e| Error::new(e.to_string())),
        Rule::string => pair.into_inner().next().map(|p| ExpType::STRING(p.as_str().to_string())).ok_or(Error::new("string parse error".to_string())),
        _ => unreachable!(),
    }
}

fn parse_function(pair: Pair<'_, Rule>, cache: &mut Cache) -> Result<ExpType, Error> {
    let mut function_name = String::new();
    let mut params: Vec<ExpType> = Vec::new();
    pair.into_inner().for_each(|item| {
        match item.as_rule() {
            Rule::function_name => {
                function_name = item.as_str().to_string();
            },
            Rule::function_arg => {
                params.push(parse_value(item.into_inner().next().unwrap(), cache).unwrap())
            },
            _ => unreachable!()
        }
    });
    cache.call_function(function_name.as_str(), params)
}