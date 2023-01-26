mod formatter;

use formatter::{ deduct_base, format_output, parse_number };
use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
/// A simple base conversion tool
struct CliInput
{
    /// The input number
    number_input: String,
    /// The output base
    output_base: u32,
    /// The input base. If omitted, the base is tried to be deducted. Looks if the input is starting with 0b or 0x and otherwise tries to use decimal base
    input_base: Option<u32>
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

