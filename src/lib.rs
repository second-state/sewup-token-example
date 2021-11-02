use sewup::types::Address;
use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[ewasm_constructor]
fn constructor() {
    sewup::token::erc20::mint("8663DBF0cC68AaF37fC8BA262F2df4c666a41993", 1000);
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

#[ewasm_main]
fn main() -> anyhow::Result<()> {
    let contract = sewup::primitives::Contract::new()?;
    match contract.get_function_selector()? {
        // Reuse the signature
        sewup::token::erc20::BALANCE_OF_SIG => balnace_of_wrapper(&contract),
        sewup::token::erc20::TRANSFER_SIG => sewup::token::erc20::transfer(&contract),
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
        let balance_input = hex!("1cCA28600d7491365520B31b466f88647B9839eC");
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
    }
}
