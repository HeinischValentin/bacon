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
            write!(f, "Can't derive base from given character.")
    }
}

#[derive(Parser)]
struct CliInput
{
    input_base: String,
    output_base: String,
    number_input: String
}

enum Base
{
    Binary,
    Decimal,
    Hexadecimal
}

fn parse_base(input: String) -> Result<Base, BaseError>
{
    let base = match input.as_str()
    {
        "b" => Ok(Base::Binary),
        "d" => Ok(Base::Decimal),
        "x" => Ok(Base::Hexadecimal),
        _   => Err(BaseError)
    };

    return base;
}

fn get_base_dec(base: Base) -> u32
{
    let dec = match base
    {
        Base::Binary => 2,
        Base::Decimal => 10,
        Base::Hexadecimal => 16
    };

    return dec;
}

fn format_output(num: i64, base: Base) -> String
{
    match base
    {
        Base::Binary => return format!("{num:#b}"),
        Base::Decimal => return format!("{}", num),
        Base::Hexadecimal => return format!("{num:#x}")
    };
}

fn parse_number(input: String, base: Base) -> Result<i64, ParseIntError>
{
    let input_no_prefix = input.trim_start_matches("0b").trim_start_matches("0x");
    let base_dec = get_base_dec(base);
    match i64::from_str_radix(input_no_prefix, base_dec)
    {
        Ok(parsed_int) => return Ok(parsed_int),
        Err(e) => return Err(e)
    };
}

fn main()
{
    let args = CliInput::parse();

    let parsed_in_base = match parse_base(args.input_base)
    {
        Ok(parsed_base) => parsed_base,
        Err(_e) =>
        {
            println!("Invalid usage! Can't parse given input base.");
            return;
        }
    };

    let parsed_int = match parse_number(args.number_input, parsed_in_base)
    {
        Ok(parsed_int) => parsed_int,
        Err(_e) =>
        {
            println!("Invalid usage! Can't parse given input number.");
            return;
        }
    };

    let parsed_out_base = match parse_base(args.output_base)
    {
        Ok(parsed_base) => parsed_base,
        Err(_e) =>
        {
            println!("Invalid usage! Can't parse given output base.");
            return;
        }
    };
    
    let output = format_output(parsed_int, parsed_out_base);
    println!("{}", output);
}

