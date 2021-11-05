use std::convert::TryInto;

use sewup::types::Address;
use sewup_derive::{
    ewasm_call_only_by, ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test,
};

#[ewasm_constructor]
fn constructor() {
    sewup::token::erc20::mint("8663DBF0cC68AaF37fC8BA262F2df4c666a41993", 1000); // admin
    sewup::token::erc20::mint("1cCA28600d7491365520B31b466f88647B9839eC", 1000);
}

#[ewasm_fn]
fn balnace_of_wrapper(c: &sewup::primitives::Contract) {
    let caller_address = sewup::utils::caller();
    let query_address: Address = sewup::helpers::copy_into_address(&c.input_data[16..36]).into();
    if caller_address == Address::from_str("8663DBF0cC68AaF37fC8BA262F2df4c666a41993").unwrap()
        || caller_address == query_address
    {
        sewup::token::erc20::balance_of(c)
    }
}

#[ewasm_fn("1249c58b", {
    "constant": false,
    "inputs": [],
    "name": "mint",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "payable": false,
    "stateMutability": "nonpayable",
    "type": "function",
})]
fn mint_to_admin() {
    let caller_address = sewup::utils::caller();
    if caller_address == Address::from_str("8663DBF0cC68AaF37fC8BA262F2df4c666a41993").unwrap() {
        sewup::token::erc20::mint("8663DBF0cC68AaF37fC8BA262F2df4c666a41993", 1000);
    }
}

#[ewasm_fn("1395e640", {
    "constant": false,
    "inputs": [
        { "internalType": "address", "name": "recipient", "type": "address" },
        { "internalType": "uint256", "name": "amount", "type": "uint256" }
    ],
    "name": "reduce",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "payable": false,
    "stateMutability": "nonpayable",
    "type": "function"
})]
fn reduce_to(contract: &sewup::primitives::Contract) {
    let caller_address = sewup::utils::caller();
    if caller_address == Address::from_str("8663DBF0cC68AaF37fC8BA262F2df4c666a41993").unwrap() {
        let user = sewup::token::helpers::copy_into_address(&contract.input_data[16..36]);
        let value = {
            let buffer: [u8; 32] = contract.input_data[36..68].try_into().unwrap();
            sewup::token::helpers::copy_into_storage_value(&buffer)
        };
        sewup::token::helpers::set_balance(&user, &value);
    }
}

#[ewasm_main]
fn main() -> anyhow::Result<()> {
    let contract = sewup::primitives::Contract::new()?;
    match contract.get_function_selector()? {
        // Reuse the signature
        sewup::token::erc20::BALANCE_OF_SIG => balnace_of_wrapper(&contract),
        sewup::token::erc20::TRANSFER_SIG => sewup::token::erc20::transfer(&contract),
        ewasm_fn_sig!(mint_to_admin) => mint_to_admin(),
        ewasm_fn_sig!(reduce_to) => reduce_to(&contract),
        _ => panic!("unknown handle"),
    };
    Ok(())
}

#[ewasm_test]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sewup::erc20::{BALANCE_OF_SIG, TRANSFER_SIG};
    use sewup_derive::{ewasm_assert_eq, ewasm_assert_ok};

    #[ewasm_test]
    fn test_get_greeting() {
        let mut balance_input = hex!("1cCA28600d7491365520B31b466f88647B9839eC");
        let mut input_data = vec![0u8, 0u8, 0u8, 0u8];
        input_data.append(&mut balance_input.to_vec());
        ewasm_assert_eq!(balance_of(input_data), vec![]);

        ewasm_assert_eq!(
            balance_of(input_data) by "1cCA28600d7491365520B31b466f88647B9839eC",
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 232
            ]
        );

        ewasm_assert_eq!(
            balance_of(input_data) by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 232
            ]
        );

        let transfer_recipent = hex!("0000000000000000000000000000000000000001");
        let transfer_value =
            hex!("0000000000000000000000000000000000000000000000000000000000000009");
        input_data = vec![0u8, 0u8, 0u8, 0u8];
        input_data.append(&mut transfer_recipent.to_vec());
        input_data.append(&mut transfer_value.to_vec());

        ewasm_assert_eq!(
            transfer(input_data) by "1cCA28600d7491365520B31b466f88647B9839eC",
            vec![]
        );

        let balance_input = hex!("1cCA28600d7491365520B31b466f88647B9839eC");
        let mut input_data = vec![0u8, 0u8, 0u8, 0u8];
        input_data.append(&mut balance_input.to_vec());
        ewasm_assert_eq!(
            balance_of(input_data) by "1cCA28600d7491365520B31b466f88647B9839eC",
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 223
            ]
        );

        ewasm_assert_eq!(
            mint_to_admin() by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            vec![]
        );

        let balance_input = hex!("8663DBF0cC68AaF37fC8BA262F2df4c666a41993");
        let mut input_data = vec![0u8, 0u8, 0u8, 0u8];
        input_data.append(&mut balance_input.to_vec());
        ewasm_assert_eq!(balance_of(input_data), vec![]);

        ewasm_assert_eq!(
            balance_of(input_data) by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 232
            ]
        );

        let reduce_recipent = hex!("1cCA28600d7491365520B31b466f88647B9839eC");
        let reduce_value = hex!("0000000000000000000000000000000000000000000000000000000000000099");
        input_data = vec![0u8, 0u8, 0u8, 0u8];
        input_data.append(&mut reduce_recipent.to_vec());
        input_data.append(&mut reduce_value.to_vec());

        ewasm_assert_eq!(
            reduce_to(input_data) by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            vec![]
        );

        let balance_input = hex!("1cCA28600d7491365520B31b466f88647B9839eC");
        let mut input_data = vec![0u8, 0u8, 0u8, 0u8];
        input_data.append(&mut balance_input.to_vec());
        ewasm_assert_eq!(
            balance_of(input_data) by "1cCA28600d7491365520B31b466f88647B9839eC",
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 153
            ]
        );
    }
}
