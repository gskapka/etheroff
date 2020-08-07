use ethereum_types::U256;
use crate::lib::{
    types::{
        Byte,
        Bytes,
        Result,
    },
    errors::AppError,
    constants::PRIVATE_KEY_HEX_LENGTH,
};

pub fn convert_dec_str_to_u256_with_err_msg(dec_str: &str, err_msg: &str) -> Result<U256> {
    match U256::from_dec_str(dec_str) {
        Ok(u256) => Ok(u256),
        Err(_) => Err(AppError::Custom(err_msg.to_string())),
    }
}

fn strip_new_lines_from_str(string: String) -> String {
    string.replace("\n", "")
}

pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}

fn bytes_to_utf8_str(bytes: &[Byte]) -> Result<String> {
    Ok(std::str::from_utf8(bytes)?.to_string())
}

pub fn convert_bytes_to_string_with_no_new_lines(bytes: &[Byte]) -> Result<String> {
    bytes_to_utf8_str(bytes).map(strip_new_lines_from_str)
}

pub fn maybe_strip_hex_prefix(hex: &str) -> Result<String> {
    let lowercase_hex_prefix = "0x";
    let uppercase_hex_prefix = "0X";
    match hex.starts_with(lowercase_hex_prefix) || hex.starts_with(uppercase_hex_prefix) {
        true => Ok(hex.trim_start_matches(lowercase_hex_prefix).trim_start_matches(uppercase_hex_prefix).to_string()),
        false => Ok(hex.to_string()),
    }
}

pub fn decode_hex_with_err_msg(hex: &str, err_msg: &str) -> Result<Bytes> {
    match hex::decode(hex) {
        Ok(bytes) => Ok(bytes),
        Err(_) => Err(AppError::Custom(err_msg.to_string()))
    }
}

pub fn validate_eth_private_key_hex_length(hex: &str) -> Result<String> { // TODO test
    match hex.chars().count() == PRIVATE_KEY_HEX_LENGTH {
        true => Ok(hex.to_string()),
        false => Err(AppError::Custom(
            format!("✘ Your private key must be {} hex chars in length!", PRIVATE_KEY_HEX_LENGTH)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_strip_hex_prefix() {
        let prefixed_hex = "0xc0ffee";
        let expected_result = "c0ffee";
        let result = maybe_strip_hex_prefix(&prefixed_hex).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_not_strip_non_existing_hex_prefix() {
        let prefixed_hex = "c0ffee";
        let result = maybe_strip_hex_prefix(&prefixed_hex).unwrap();
        assert_eq!(result, prefixed_hex);
    }

    #[test]
    fn should_validate_valid_hex() {
        let valid_hex = "c0ffee";
        let expected_result = hex::decode(valid_hex).unwrap();
        let err_msg = "Could not decode hex in test!";
        let result = decode_hex_with_err_msg(&valid_hex, err_msg).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_error_when_validating_invalid_hex() {
        let invalid_hex = "coffee";
        let expected_error = "Could not decode hex - please check your input!";
        match decode_hex_with_err_msg(&invalid_hex, &expected_error) {
            Err(AppError::Custom(err)) => assert_eq!(err, expected_error),
            Err(e) => panic!("Wrong error recieved: {}", e),
            Ok(_) => panic!("Should not have succeeded!"),
        }
    }
}
