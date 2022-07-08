use std::i64;
use std::num::ParseIntError;
use clap::Parser;

#[derive(Parser)]
struct CliInput
{
    input_base: String,
    output_base: String,
    number_input: String
}

fn parse_hex(input: String) -> Result<i64, ParseIntError>
{
    let input_no_prefix = input.trim_start_matches("0x");
    match i64::from_str_radix(input_no_prefix, 16)
    {
        Ok(parsed_int) => return Ok(parsed_int),
        Err(e) => return Err(e)
    };
}


fn parse_dec(input: String) -> Result<i64, ParseIntError>
{
    match input.parse::<i64>()
    {
        Ok(parsed_int) => return Ok(parsed_int),
        Err(e) => return Err(e)
    };
}

fn parse_bin(input: String) -> Result<i64, ParseIntError>
{
    let input_no_prefix = input.trim_start_matches("0b");
    match i64::from_str_radix(input_no_prefix, 2)
    {
        Ok(parsed_int) => return Ok(parsed_int),
        Err(e) => return Err(e)
    };
}

fn parse_number(input: String, base: i8) -> Result<i64, ParseIntError>
{
    match base
    {
        2 => return parse_bin(input),
        10 => return parse_dec(input),
        16 => return parse_hex(input),
        _ => return parse_bin(String::from("error"))          // TODO: Create Error Type
    };
}

fn parse_base(input: String) -> Result<i8, ParseIntError>
{
    match input.parse::<i8>()
    {
        Ok(parsed_base) => return Ok(parsed_base),
        Err(e) => return Err(e)
    };
}

fn main()
{
    let args = CliInput::parse();

    let parsed_base = match parse_base(args.input_base)
    {
        Ok(parsed_base) => parsed_base,
        Err(_e) =>
        {
            println!("Invalid usage! Can't parse given input base.");
            return;
        }
    };

    let parsed_int = match parse_number(args.number_input, parsed_base)
    {
        Ok(parsed_int) => parsed_int,
        Err(_e) =>
        {
            println!("Invalid usage! Can't parse given input number.");
            return;
        }
    };
    println!("{}", parsed_int);
}

