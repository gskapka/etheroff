use ethereum_types::U256;

use crate::lib::{
    constants::{ETH_NUM_DECIMALS, ONE_GWEI},
    types::Result,
};

fn trim_newline(s: &mut String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s.to_string()
}

pub fn get_user_input() -> Result<String> {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;
    Ok(trim_newline(&mut user_input))
}

fn get_u256_or_return_custom_error(dec_str: &str, err: String) -> Result<U256> {
    match U256::from_dec_str(dec_str) {
        Ok(u256) => Ok(u256),
        Err(_) => Err(err.into()),
    }
}

fn right_pad(str_to_pad: &str, char_to_pad: &char, length_to_pad_to: usize) -> String {
    let str_to_pad_len = str_to_pad.chars().count();
    let mut result_string = str_to_pad.to_string();
    if str_to_pad_len >= length_to_pad_to {
        str_to_pad.to_string()
    } else {
        for _ in 0..length_to_pad_to - str_to_pad_len {
            result_string.push(*char_to_pad)
        }
        result_string
    }
}

fn right_pad_to_wei(str_to_pad: &str) -> String {
    right_pad(str_to_pad, &'0', ETH_NUM_DECIMALS)
}

pub fn convert_eth_amount_str_to_u256(eth_amount_str: &str) -> Result<U256> {
    let err_msg = format!("✘ Could not parse ETH amount from: {}!", eth_amount_str);
    let amount_no_whitespace: String = eth_amount_str.split_whitespace().collect();
    match amount_no_whitespace.contains('.') {
        false => match amount_no_whitespace.len() {
            0 => Ok(U256::from_dec_str("0")?),
            _ => get_u256_or_return_custom_error(&format!("{}000000000000000000", amount_no_whitespace), err_msg),
        },
        true => {
            let components: Vec<&str> = amount_no_whitespace.split('.').collect();
            match components.len() {
                1 => get_u256_or_return_custom_error(&format!("{}000000000000000000", amount_no_whitespace), err_msg),
                2 => get_u256_or_return_custom_error(
                    &format!("{}{}", components[0], right_pad_to_wei(&components[1])),
                    err_msg,
                ),
                _ => Err(err_msg.into()),
            }
        },
    }
}

pub fn convert_eth_gas_price_gwei_to_wei(gas_price_gwei: f64) -> Result<U256> {
    let rounded = (gas_price_gwei * ONE_GWEI as f64).round() / ONE_GWEI as f64;
    match U256::from_dec_str(&format!("{}", ((rounded * ONE_GWEI as f64) as u64))) {
        Ok(u256) => Ok(u256),
        Err(_) => Err(format!("✘ Could not convert {} to gas limit in wei!", gas_price_gwei).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::errors::AppError;

    fn convert_eth_string_and_assert_result(eth_amount_str: &str, expected_result: U256) {
        let result = convert_eth_amount_str_to_u256(eth_amount_str).unwrap();
        assert_eq!(result, expected_result);
    }

    fn convert_eth_string_and_assert_error(eth_amount_str: &str) {
        let expected_error = format!("✘ Could not parse ETH amount from: {}!", eth_amount_str);
        match convert_eth_amount_str_to_u256(eth_amount_str) {
            Err(AppError::Custom(e)) => assert_eq!(e, expected_error),
            Err(e) => panic!("Wrong err: {}", e),
            Ok(_) => panic!("Should not have succeeded!"),
        }
    }

    #[test]
    fn should_convert_decimal_string_to_u256() {
        convert_eth_string_and_assert_result("0.000000000000000001", U256::from_dec_str("1").unwrap());
        convert_eth_string_and_assert_result("0.000000000000000012", U256::from_dec_str("12").unwrap());
        convert_eth_string_and_assert_result("0.000000000000000123", U256::from_dec_str("123").unwrap());
        convert_eth_string_and_assert_result("0.000000000000001234", U256::from_dec_str("1234").unwrap());
        convert_eth_string_and_assert_result("0.000000000000012345", U256::from_dec_str("12345").unwrap());
        convert_eth_string_and_assert_result("0.000000000000123456", U256::from_dec_str("123456").unwrap());
        convert_eth_string_and_assert_result("0.000000000001234567", U256::from_dec_str("1234567").unwrap());
        convert_eth_string_and_assert_result("0.000000000012345678", U256::from_dec_str("12345678").unwrap());
        convert_eth_string_and_assert_result("0.000000000123456789", U256::from_dec_str("123456789").unwrap());
        convert_eth_string_and_assert_result("0.000000001234567891", U256::from_dec_str("1234567891").unwrap());
        convert_eth_string_and_assert_result("0.000000012345678912", U256::from_dec_str("12345678912").unwrap());
        convert_eth_string_and_assert_result("0.000000123456789123", U256::from_dec_str("123456789123").unwrap());
        convert_eth_string_and_assert_result("0.000001234567891234", U256::from_dec_str("1234567891234").unwrap());
        convert_eth_string_and_assert_result("0.000012345678912345", U256::from_dec_str("12345678912345").unwrap());
        convert_eth_string_and_assert_result("0.000123456789123456", U256::from_dec_str("123456789123456").unwrap());
        convert_eth_string_and_assert_result("0.001234567891234567", U256::from_dec_str("1234567891234567").unwrap());
        convert_eth_string_and_assert_result("0.012345678912345678", U256::from_dec_str("12345678912345678").unwrap());
        convert_eth_string_and_assert_result(
            "0.123456789123456789",
            U256::from_dec_str("123456789123456789").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.123456789123456789",
            U256::from_dec_str("1123456789123456789").unwrap(),
        );

        convert_eth_string_and_assert_result(
            "1.000000000000000001",
            U256::from_dec_str("1000000000000000001").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000000000000012",
            U256::from_dec_str("1000000000000000012").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000000000000123",
            U256::from_dec_str("1000000000000000123").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000000000001234",
            U256::from_dec_str("1000000000000001234").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000000000012345",
            U256::from_dec_str("1000000000000012345").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000000000123456",
            U256::from_dec_str("1000000000000123456").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000000001234567",
            U256::from_dec_str("1000000000001234567").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000000012345678",
            U256::from_dec_str("1000000000012345678").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000000123456789",
            U256::from_dec_str("1000000000123456789").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000001234567891",
            U256::from_dec_str("1000000001234567891").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000012345678912",
            U256::from_dec_str("1000000012345678912").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000000123456789123",
            U256::from_dec_str("1000000123456789123").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000001234567891234",
            U256::from_dec_str("1000001234567891234").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000012345678912345",
            U256::from_dec_str("1000012345678912345").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.000123456789123456",
            U256::from_dec_str("1000123456789123456").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.001234567891234567",
            U256::from_dec_str("1001234567891234567").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.012345678912345678",
            U256::from_dec_str("1012345678912345678").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.123456789123456789",
            U256::from_dec_str("1123456789123456789").unwrap(),
        );

        convert_eth_string_and_assert_result("1.1", U256::from_dec_str("1100000000000000000").unwrap());
        convert_eth_string_and_assert_result("1.12", U256::from_dec_str("1120000000000000000").unwrap());
        convert_eth_string_and_assert_result("1.123", U256::from_dec_str("1123000000000000000").unwrap());
        convert_eth_string_and_assert_result("1.1234", U256::from_dec_str("1123400000000000000").unwrap());
        convert_eth_string_and_assert_result("1.12345", U256::from_dec_str("1123450000000000000").unwrap());
        convert_eth_string_and_assert_result("1.123456", U256::from_dec_str("1123456000000000000").unwrap());
        convert_eth_string_and_assert_result("1.1234567", U256::from_dec_str("1123456700000000000").unwrap());
        convert_eth_string_and_assert_result("1.12345678", U256::from_dec_str("1123456780000000000").unwrap());
        convert_eth_string_and_assert_result("1.123456789", U256::from_dec_str("1123456789000000000").unwrap());
        convert_eth_string_and_assert_result("1.1234567891", U256::from_dec_str("1123456789100000000").unwrap());
        convert_eth_string_and_assert_result("1.12345678912", U256::from_dec_str("1123456789120000000").unwrap());
        convert_eth_string_and_assert_result("1.123456789123", U256::from_dec_str("1123456789123000000").unwrap());
        convert_eth_string_and_assert_result("1.1234567891234", U256::from_dec_str("1123456789123400000").unwrap());
        convert_eth_string_and_assert_result("1.12345678912345", U256::from_dec_str("1123456789123450000").unwrap());
        convert_eth_string_and_assert_result("1.123456789123456", U256::from_dec_str("1123456789123456000").unwrap());
        convert_eth_string_and_assert_result("1.1234567891234567", U256::from_dec_str("1123456789123456700").unwrap());
        convert_eth_string_and_assert_result(
            "1.12345678912345678",
            U256::from_dec_str("1123456789123456780").unwrap(),
        );
        convert_eth_string_and_assert_result(
            "1.123456789123456789",
            U256::from_dec_str("1123456789123456789").unwrap(),
        );

        convert_eth_string_and_assert_result(".1", U256::from_dec_str("100000000000000000").unwrap());
        convert_eth_string_and_assert_result(".12", U256::from_dec_str("120000000000000000").unwrap());
        convert_eth_string_and_assert_result(".123", U256::from_dec_str("123000000000000000").unwrap());
        convert_eth_string_and_assert_result(".1234", U256::from_dec_str("123400000000000000").unwrap());
        convert_eth_string_and_assert_result(".12345", U256::from_dec_str("123450000000000000").unwrap());
        convert_eth_string_and_assert_result(".123456", U256::from_dec_str("123456000000000000").unwrap());
        convert_eth_string_and_assert_result(".1234567", U256::from_dec_str("123456700000000000").unwrap());
        convert_eth_string_and_assert_result(".12345678", U256::from_dec_str("123456780000000000").unwrap());
        convert_eth_string_and_assert_result(".123456789", U256::from_dec_str("123456789000000000").unwrap());
        convert_eth_string_and_assert_result(".1234567891", U256::from_dec_str("123456789100000000").unwrap());
        convert_eth_string_and_assert_result(".12345678912", U256::from_dec_str("123456789120000000").unwrap());
        convert_eth_string_and_assert_result(".123456789123", U256::from_dec_str("123456789123000000").unwrap());
        convert_eth_string_and_assert_result(".1234567891234", U256::from_dec_str("123456789123400000").unwrap());
        convert_eth_string_and_assert_result(".12345678912345", U256::from_dec_str("123456789123450000").unwrap());
        convert_eth_string_and_assert_result(".123456789123456", U256::from_dec_str("123456789123456000").unwrap());
        convert_eth_string_and_assert_result(".1234567891234567", U256::from_dec_str("123456789123456700").unwrap());
        convert_eth_string_and_assert_result(".12345678912345678", U256::from_dec_str("123456789123456780").unwrap());
        convert_eth_string_and_assert_result(".123456789123456789", U256::from_dec_str("123456789123456789").unwrap());
        convert_eth_string_and_assert_result("", U256::from_dec_str("0").unwrap());

        convert_eth_string_and_assert_result("1", U256::from_dec_str("1000000000000000000").unwrap());
        convert_eth_string_and_assert_result("1337.1337", U256::from_dec_str("1337133700000000000000").unwrap());
        convert_eth_string_and_assert_result(
            "1337.000000000000001337",
            U256::from_dec_str("1337000000000000001337").unwrap(),
        );
        convert_eth_string_and_assert_result("13.37 ", U256::from_dec_str("13370000000000000000").unwrap());

        convert_eth_string_and_assert_error("qwerty");
        convert_eth_string_and_assert_error("qwerty.1");
        convert_eth_string_and_assert_error("1.qwerty");
        convert_eth_string_and_assert_error("1.1.");

        convert_eth_string_and_assert_result(" ", U256::from_dec_str("0").unwrap());
    }

    #[test]
    fn should_convert_gas_price_gwei_to_gas_price_wei() {
        let expected_result = U256::from_dec_str("21100000000").unwrap();
        let gas_price_gwei: f64 = 21.1;
        let result = convert_eth_gas_price_gwei_to_wei(gas_price_gwei).unwrap();
        assert_eq!(result, expected_result)
    }

    #[test]
    fn should_convert_gas_price_gwei_to_wei_even_if_too_many_decimals_provided() {
        let expected_result = U256::from_dec_str("21123456789").unwrap();
        let gas_price_gwei: f64 = 21.123_456_789_123_455;
        let result = convert_eth_gas_price_gwei_to_wei(gas_price_gwei).unwrap();
        assert_eq!(result, expected_result)
    }
}
