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

fn format_output(num: i64, base: String) -> String
{
    match base.as_str()
    {
        "b" => return format!("{num:#b}"),
        "d" => return format!("{}", num),
        "x" => return format!("{num:#x}"),
        _   => return "Error! The given base is not implemented (yet)!".to_string()
    };
}

fn parse_number(input: String, base: u32) -> Result<i64, ParseIntError>
{
    let input_no_prefix = input.trim_start_matches("0b").trim_start_matches("0x");
    match i64::from_str_radix(input_no_prefix, base)
    {
        Ok(parsed_int) => return Ok(parsed_int),
        Err(e) => return Err(e)
    };
}

fn parse_base(input: String) -> Result<u32, ParseIntError>
{
    match input.parse::<u32>()
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
    
    let output = format_output(parsed_int, args.output_base);
    println!("{}", output);
}

