use std::{io::stdin, str::FromStr};

use base1112031::{FromBase1112031, ToBase1112031};
use num_bigint::BigUint;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        help();
        return;
    }
    match args[1].as_str() {
        "--to-base1112031" => to_base1112031(),
        "--to-base10" => to_base10(),
        _ => help(),
    }
}

fn to_base1112031() {
    let mut buf = String::new();
    while let Ok(o) = stdin().read_line(&mut buf) {
        if o == 0 {
            break;
        }

        let input = match buf.strip_suffix('\n') {
            Some(s) => s,
            None => &buf,
        };
        if input.is_empty() {
            continue;
        }

        let input = BigUint::from_str(input).unwrap();
        println!("{}", input.to_base1112031::<String>().unwrap());

        buf.clear();
    }
}

fn to_base10() {
    let mut buf = String::new();
    while let Ok(o) = stdin().read_line(&mut buf) {
        if o == 0 {
            break;
        }

        let input = match buf.strip_suffix('\n') {
            Some(s) => s,
            None => &buf,
        };
        if input.is_empty() {
            continue;
        }

        let result: BigUint = FromBase1112031::from_base1112031(input).unwrap();
        println!("{}", result);

        buf.clear();
    }
}

fn help() {
    println!("base1112031 generator");
    println!("Command line options");
    println!(
        "--to-base1112031 : Receives a decimal number from stdin and converts it to base1112031."
    );
    println!("--to-base10 : Receives a base1112031 from stdin and converts it to decimal number.");
}
