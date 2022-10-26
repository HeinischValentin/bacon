use std::char::from_digit;
use std::i64;
use std::num::ParseIntError;
use std::fmt;
use clap::Parser;

#[derive(Debug, Clone)]
struct BaseError;

impl fmt::Display for BaseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
            write!(f, "The given base could not be calculated.")
    }
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
/// A simple BaseConversion tool
struct CliInput
{
    /// The input number
    number_input: String,
    /// The output base
    output_base: u32,
    /// The input base. If omitted, the base is tried to be deducted. Looks if the input is starting with 0b or 0x and otherwise tries to use decimal base
    input_base: Option<u32>
}

fn format_output(num: u64, base: u32) -> Result<String, BaseError>
{
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
        }
        else {
            max_pow -= 1;
            break;
        }
    }
    
    let mut res = Vec::new();
    let mut remainder = num;
    for i in (0..=max_pow).rev() {
        res.push(remainder/base.pow(i) as u64);
        remainder = remainder % base.pow(i) as u64;
    }

    let mut result_string = String::new();
    for n in res.iter() {
        result_string.push(from_digit(*n as u32, base as u32).expect("Error!"));
    }

    Ok(result_string)
}

fn parse_number(input: &String, base: u32) -> Result<i64, ParseIntError>
{
    let input_no_prefix = input.trim_start_matches("0b").trim_start_matches("0x");
    match i64::from_str_radix(input_no_prefix, base as u32)
    {
        Ok(parsed_int) => return Ok(parsed_int),
        Err(e) => return Err(e)
    };
}

fn deduct_base(input: &String) -> u32
{
    if input.starts_with("0b") {
        return 2;
    }
    else if input.starts_with("00") {
        return 8;
    }
    else if input.starts_with("0x") {
        return 16;
    }
    else {
        return 10;
    }
}

fn main()
{
    let args = CliInput::parse();

    let base_in = match args.input_base {
        Some(b) => b,
        None    => deduct_base(&args.number_input)
    };

    let parsed_int = match parse_number(&args.number_input, base_in)
    {
        Ok(parsed_int) => parsed_int,
        Err(_e) =>
        {
            println!("Invalid usage! Can't parse given input number.");
            return;
        }
    };

    let output = format_output(parsed_int as u64, args.output_base).expect("Error aua!");
    println!("{}", output);
}

