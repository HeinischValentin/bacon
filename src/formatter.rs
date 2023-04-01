use std::char::from_digit;
use std::fmt;
use std::i64;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct BaseError;

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The given base could not be calculated.")
    }
}

pub fn format_output(num: u64, base: u32) -> Result<String, BaseError> {
    if base < 2 || base > 36 {
        return Err(BaseError);
    }

    if base == 10 {
        return Ok(format!("{}", num));
    }

    let mut max_pow = 0;
    loop {
        let div = num as f64 / base.pow(max_pow) as f64;
        if div >= 1.0 {
            max_pow += 1;
        } else {
            max_pow -= 1;
            break;
        }
    }

    let mut res = Vec::new();
    let mut remainder = num;
    for i in (0..=max_pow).rev() {
        res.push(remainder / base.pow(i) as u64);
        remainder = remainder % base.pow(i) as u64;
    }

    let mut result_string = String::new();
    for n in res.iter() {
        result_string.push(from_digit(*n as u32, base as u32).expect("Error!"));
    }

    Ok(result_string)
}

pub fn parse_number(input: &String, base: u32) -> Result<i64, ParseIntError> {
    let input_no_prefix = input.trim_start_matches("0b").trim_start_matches("0x");
    match i64::from_str_radix(input_no_prefix, base) {
        Ok(parsed_int) => return Ok(parsed_int),
        Err(e) => return Err(e),
    };
}

pub fn deduct_base(input: &String) -> u32 {
    if input.starts_with("0b") {
        return 2;
    } else if input.starts_with("0o") {
        return 8;
    } else if input.starts_with("0x") {
        return 16;
    } else {
        return 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_output() {
        let mut in_num: u64 = 10;
        let mut in_base: u32 = 2;
        let out_string: String = format_output(in_num, in_base).expect("");
        assert_eq!(out_string, "1010".to_string());

        in_num = 175;
        in_base = 16;
        let out_string: String = format_output(in_num, in_base).expect("");
        assert_eq!(out_string, "af".to_string());

        in_num = 255;
        in_base = 10;
        let out_string: String = format_output(in_num, in_base).expect("");
        assert_eq!(out_string, "255".to_string());
    }

    #[test]
    #[should_panic]
    fn test_format_output_panic() {
        let mut in_num: u64 = 10;
        let mut in_base: u32 = 1;
        let mut _out_string: String = format_output(in_num, in_base).expect("");

        in_num = 175;
        in_base = 37;
        _out_string = format_output(in_num, in_base).expect("");
    }

    #[test]
    fn test_parse_number() {
        let mut in_string: String = "465".to_string();
        let mut in_base: u32 = 10;
        let mut out_num: i64 = parse_number(&in_string, in_base).expect("");
        assert_eq!(out_num, 465);

        in_string = "11111111".to_string();
        in_base = 2;
        out_num = parse_number(&in_string, in_base).expect("");
        assert_eq!(out_num, 255);

        in_string = "0b11111111".to_string();
        in_base = 2;
        out_num = parse_number(&in_string, in_base).expect("");
        assert_eq!(out_num, 255);

        in_string = "465".to_string();
        in_base = 16;
        out_num = parse_number(&in_string, in_base).expect("");
        assert_eq!(out_num, 1125);

        in_string = "0x465".to_string();
        in_base = 16;
        out_num = parse_number(&in_string, in_base).expect("");
        assert_eq!(out_num, 1125);
    }

    #[test]
    fn test_deduct_base() {
        let mut in_string: String = "0b101".to_string();
        let mut out_base: u32 = deduct_base(&in_string);
        assert_eq!(out_base, 2);

        in_string = "0o255".to_string();
        out_base = deduct_base(&in_string);
        assert_eq!(out_base, 8);

        in_string = "255".to_string();
        out_base = deduct_base(&in_string);
        assert_eq!(out_base, 10);

        in_string = "0x255".to_string();
        out_base = deduct_base(&in_string);
        assert_eq!(out_base, 16);
    }
}
