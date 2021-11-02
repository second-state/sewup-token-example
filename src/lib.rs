use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[ewasm_constructor]
fn constructor() {}

#[ewasm_fn]
fn hello() -> anyhow::Result<String> {
    Ok("hello world".to_string())
}

#[ewasm_main]
fn main() -> anyhow::Result<()> {
    let contract = sewup::primitives::Contract::new()?;
    match contract.get_function_selector()? {
        ewasm_fn_sig!(hello) => hello()?,
        _ => panic!("unknown handle"),
    };
    Ok(())
}

#[ewasm_test]
mod tests {
    use super::*;
    use sewup_derive::{ewasm_assert_eq, ewasm_assert_ok};

    #[ewasm_test]
    fn test_get_greeting() {
        // The default mode does not use anything successful return from Rust
        // The only thing is return ok or not
        ewasm_assert_eq!(hello(), vec![]);
        ewasm_assert_ok!(hello());
    }
}
