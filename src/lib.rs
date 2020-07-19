#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]

extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;
extern crate alloc;
extern crate very_simple_json_parser;

pub mod jsonParser {
    use pwasm_ethereum;
    use pwasm_abi::types::*;
    use pwasm_abi_derive::eth_abi;
    use alloc::collections::BTreeMap;
    use very_simple_json_parser::parse;
    use alloc::format;
    #[eth_abi(TokenEndpoint, TokenClient)]
    pub trait JsonParserInterface {
        fn constructor(&mut self);
        #[constant]
        fn checkFieldIntegrity(&mut self,  _str: String) -> String;
    }


    pub struct JsonParserContract;

    impl JsonParserInterface for JsonParserContract {
        fn constructor(&mut self) {
        
        }

        fn checkFieldIntegrity(&mut self, str: String) -> String {


            match parse(str) {
                Err(s) => String::from(format!("error {}", s)),
                Ok(map) => {
                    if map["accountName"].len() < 10 {
                        return String::from("pass");
                    }
                    String::from("fail")
                }
            }
        }
    }

}
// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

#[no_mangle]
pub fn call() {
    let mut endpoint = jsonParser::TokenEndpoint::new(jsonParser::JsonParserContract{});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {
    let mut endpoint = jsonParser::TokenEndpoint::new(jsonParser::JsonParserContract{});
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}


// #[cfg(test)]
// #[allow(non_snake_case)]
// mod tests {
//     extern crate pwasm_test;
//     extern crate std;
//     use super::*;
//     use self::pwasm_test::{ext_reset, ext_get};
//     use jsonParser::JsonParserInterface;

//     #[test]
//     fn should_succeed_transfering_1000_from_owner_to_another_address() {
//         let mut contract = jsonParser::JsonParserContract{};
        

//         contract.constructor();
//         assert_eq!(contract.checkFieldIntegrity(r#"{"age":1111}"#.into()), true);
//         assert_eq!(ext_get().logs().len(), 0);
//     }
// }