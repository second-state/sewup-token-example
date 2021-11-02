use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[ewasm_constructor]
fn constructor() {
    sewup::token::erc20::mint("8663DBF0cC68AaF37fC8BA262F2df4c666a41993", 1000);
}

#[ewasm_main]
fn main() -> anyhow::Result<()> {
    let contract = sewup::primitives::Contract::new()?;
    match contract.get_function_selector()? {
        sewup::token::erc20::BALANCE_OF_SIG => sewup::token::erc20::balance_of(&contract),
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
        let balance_input = hex!("8663DBF0cC68AaF37fC8BA262F2df4c666a41993");
        let mut input_data = vec![0u8, 0u8, 0u8, 0u8];
        input_data.append(&mut balance_input.to_vec());
        ewasm_assert_eq!(
            balance_of(input_data),
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

        // assert the transfer() function of contract and call by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993"
        ewasm_assert_eq!(
            transfer(input_data) by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            vec![]
        );

        let balance_input = hex!("8663DBF0cC68AaF37fC8BA262F2df4c666a41993");
        let mut input_data = vec![0u8, 0u8, 0u8, 0u8];
        input_data.append(&mut balance_input.to_vec());
        ewasm_assert_eq!(
            balance_of(input_data),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 223
            ]
        );
    }
}
