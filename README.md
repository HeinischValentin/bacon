# bacon
Simple BAse CONversion tool for the command line

[![Rust](https://github.com/HeinischValentin/bacon/actions/workflows/rust.yml/badge.svg)](https://github.com/HeinischValentin/bacon/actions/workflows/rust.yml)

## Description

If you sometimes need to convert numbers to another base and like working on the command line, this project could be interesting for you.
Bacon is a very simple tool, to convert any number, from and to any base you like.
While working on some embedded hardware projects, I eventually got lazy of switching to the web browser to convert some memory addresses or register values.
The results of my search for any tools to solve this problem only showed rather inconvenient scripts using the built-in math functions of some interpreted programming languages or the `bc` command.
Thus, I decided to write my own little program.

## Getting Started

### Dependencies

To build and run this project, you will only need a working rust toolchain.

### Installing

* Clone this repository
* Run 'cargo build --release'
* Take the executable from ./target/release/bacon to any location you want

### Executing program

To run the program, simply invoke it with your input number, the desired output base and the base of your input number.
You can also omit the input base. The program will then try to deduct it based on the given input number.
For this, the programm will take prefixes like '0x' or '0b' into consideration. Otherwise it will asume a base of 10.

```
bacon <INPUT_NUMBER> <OUTPUT_BASE> [INPUT_BASE]
```

E.g. this call will return '255':

```
bacon 0xff 10
```

This call will also return '255':

```
bacon 0xff 10 16
```

## Authors

Valentin Heinisch

## Version History

* 0.1
    * Initial Release

## License

This project is licensed under the GNU GPL v3.0 License - see the LICENSE file for details

