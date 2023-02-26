use evalexpr::*;
use regex::Captures;
use regex::Regex;
use std::{io, process};

fn main() {
    println!("Numeric?");

    let verify_parser: Regex = Regex::new(r"\((.*?)+\)/\d").unwrap();
    let mut numeric;

    // Name for clarity.
    'input: loop {
        // The following line is required to prevent mere appension (which I don't know why occurs).
        numeric = String::new();
        io::stdin().read_line(&mut numeric).unwrap();

        if *(&numeric.to_lowercase().trim()) == "exit" {
            process::exit(0);
        }

        if !verify_parser.is_match(&numeric) {
            println!("Invalid equation, try again.\nNote: no spaces permitted; must be in the form of (...)/x\nwhere `...` is an addition sequence and `x` is a positive integer.");
            continue 'input;
        }

        let captures: Captures = verify_parser.captures(&numeric).unwrap();
        let text: &str = captures.get(1).unwrap().as_str();

        let num_elements: usize = text.split(['+', '-']).count();
        let divider = (&numeric.split('/').collect::<Vec<&str>>()[1].trim())
            .parse::<usize>()
            .unwrap();

        if num_elements != divider {
            println!(
                "Number of elements does not equal divider.\nNumber elements: {}\nDivider: {}",
                &num_elements, &divider
            );
            continue 'input;
        }

        let evaluation = eval(format!("({}) * 100", &numeric).as_str()).unwrap();
        let answer = evaluation.to_string();

        println!("Number elements: {}", num_elements);
        println!("Result: {:.4}%", &answer);
    }
}