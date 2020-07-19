#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::result::Result;

struct Pair {
    key: String,
    value: String
}

fn parse_pair(chars: &Vec<char>, idx: &mut usize) -> Result<Pair, String> {
    let mut pair = Pair {key: String::new(), value: String::new()};
    if chars[usize::from(*idx)] != '\"' {
        return Result::Err(String::from("Invaild string1"));
    }
    else {
        *idx += 1;
    }
    while chars[usize::from(*idx)] != '\"' {
        pair.key.push(chars[usize::from(*idx)]);
        *idx += 1;
    }
    *idx += 1;
    while chars[usize::from(*idx)] == ' '|| chars[usize::from(*idx)] == '\n' {
        *idx += 1;
    }
    if chars[usize::from(*idx)] != ':' {
        return Result::Err(String::from("Invaild pair"));
    }
    else {
        *idx += 1;
    }
    while chars[usize::from(*idx)] == ' '|| chars[usize::from(*idx)] == '\n' {
        *idx += 1;
    }
    if chars[usize::from(*idx)] != '\"' {
        return Result::Err(String::from("Invaild string2"));
    }
    else {
        *idx += 1;
    }
    while chars[usize::from(*idx)] != '\"' {
        pair.value.push(chars[usize::from(*idx)]);
        *idx += 1;
    }
    *idx += 1;
    Ok(pair)
}


pub fn parse(str: String) -> Result<BTreeMap<String, String>, String> {
    let mut map = BTreeMap::new();
    let chars: Vec<_> = str.chars().collect();
    
    if chars.len() < 2 || chars[0] != '{' || chars[chars.len()-1] != '}' {
        return Result::Err(String::from("Invaild json1"));
    }
    let len = chars.len();
    let mut i: usize = 1;
    loop {
        while chars[i] == ' '|| chars[i] == '\n' {
            i += 1;
        }
        if chars[i] == '}' {
            break;
        }
        let p: Pair = parse_pair(&chars, &mut i)?;
        map.insert(p.key, p.value);
        while chars[i] == ' '|| chars[i] == '\n' {
            i += 1;
        }
        if chars[i] == '}' {
            break;
        }
        if chars[i] != ',' {
            return Result::Err(String::from("Invaild json2"));
        }
        else {
            i += 1;
        }
    }

    Result::Ok(map)
}

#[test]
fn test_parse() {
    let str = String::from(r#"{
        "accountName":"11111111111111111111111111111111111111111111111111",
        "account":"111111111111111111111111111111",
        "interestRate":"3",
        "startDate":"4",
        "expirationDate":"3",
        "balance":"6",
        "productName":"11111111111111111111111111111111111111111111111111",
        "productType":"11111111111111111111111111111111111111111111111111"
    }"#);
    match parse(str) {
        Err(s) => panic!(&String::from(s)),
        Ok(map) => {
            assert!(map["accountName"].len() > 10)
        }
    }
}