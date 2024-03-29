use itertools::iproduct;
use rand::Rng;
use std::cmp;
use std::collections::HashSet;

const ROMAN_NUMERALS: &[&str] = &["I", "V", "X", "L", "C", "D", "M"];
const INVALID_SUBSTRINGS: &[&str] = &[
    "LD", "VX", "IC", "DD", "VV", "VL", "IM", "DM", "LC", "VD", "IL", "XD", "VM", "XM", "LL", "VC",
    "LM", "ID", "DCD", "XLX", "XXL", "IXC", "CMD", "VIV", "CCM", "CMC", "CMM", "XXC", "IXX", "DCM",
    "CDC", "IXI", "XCC", "LXC", "VIX", "XCM", "XCL", "CCD", "IXL", "IIX", "LXL", "IXV", "IVI",
    "XCX", "IIV", "XCD", "CCCC", "XXXX", "IIII", "MMMM",
];

fn main() {
    let maximum_length = 16;
    let actual_roman_numbers = original_roman_numbers(maximum_length);
    let claimed_valid_strings = grown_valid_roman_numbers(maximum_length);

    println!(
        "Non-claimed but valid: {:?}",
        &actual_roman_numbers - &claimed_valid_strings
    );

    println!(
        "Invalid but claimed: {:?}",
        &claimed_valid_strings - &actual_roman_numbers
    );

    println!("\nTest cases in the format `INPUT : VALID, OUTPUTS`.");

    let mut rng = rand::thread_rng();
    for n in 0..16 {
        for _ in 0..1_000_000 {
            let length = rng.gen_range(cmp::max(2, n)..16);
            let candidate: String = (0..length)
                .map(|_| ROMAN_NUMERALS[rng.gen_range(0..ROMAN_NUMERALS.len())])
                .collect();
            let invalid_substrings = all_invalid_substrings(&candidate);
            if invalid_substrings.len() == n {
                let x = invalid_substrings.join(", ");
                let output = match x.is_empty() {
                    true => "VALID",
                    false => &x,
                };
                println!("{} : {}", candidate, output);
                break;
            }
        }
    }
}

fn original_roman_numbers(maximum_length: usize) -> HashSet<String> {
    (1..4000)
        .map(roman_from_decimal)
        .filter(|r| r.len() <= maximum_length)
        .collect()
}

fn grown_valid_roman_numbers(maximum_length: usize) -> HashSet<String> {
    let mut sets_of_length_n_strings: Vec<HashSet<String>> =
        vec![HashSet::new(); maximum_length + 1];

    sets_of_length_n_strings[0].insert("".to_string());

    for n in 1..=maximum_length {
        sets_of_length_n_strings[n] =
            iproduct!(sets_of_length_n_strings[n - 1].clone(), ROMAN_NUMERALS)
                .map(|tuple| format!("{}{}", tuple.0, tuple.1))
                .filter(|s| valid_roman_number(s))
                .collect();

        let length = sets_of_length_n_strings[n].len();

        println!(
            "There {} {} valid string{} of length {}",
            match length {
                1 => "is",
                _ => "are",
            },
            length,
            match length {
                1 => "",
                _ => "s",
            },
            n
        );
    }

    let grown_numbers = sets_of_length_n_strings
        .iter()
        .skip(1) // Omit the length 0 string
        .fold(HashSet::new(), |acc, set| &acc | set);

    println!("There are {} valid strings in total", grown_numbers.len());

    grown_numbers
}

fn valid_roman_number(candidate: &str) -> bool {
    all_invalid_substrings(candidate).is_empty()
}

fn all_invalid_substrings(candidate: &str) -> Vec<String> {
    INVALID_SUBSTRINGS
        .iter()
        .filter(|s| candidate.contains(&s.to_string()))
        .map(|s| s.to_string())
        .collect()
}

fn roman_from_decimal(n: usize) -> String {
    format!("{}{}{}{}", thousands(n), hundreds(n), tens(n), units(n))
}

fn units(n: usize) -> String {
    match n % 10 {
        0 => "",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        _ => unreachable!(),
    }
    .to_string()
}

fn tens(n: usize) -> String {
    match (n / 10) % 10 {
        0 => "",
        1 => "X",
        2 => "XX",
        3 => "XXX",
        4 => "XL",
        5 => "L",
        6 => "LX",
        7 => "LXX",
        8 => "LXXX",
        9 => "XC",
        _ => unreachable!(),
    }
    .to_string()
}

fn hundreds(n: usize) -> String {
    match (n / 100) % 10 {
        0 => "",
        1 => "C",
        2 => "CC",
        3 => "CCC",
        4 => "CD",
        5 => "D",
        6 => "DC",
        7 => "DCC",
        8 => "DCCC",
        9 => "CM",
        _ => unreachable!(),
    }
    .to_string()
}

fn thousands(n: usize) -> String {
    match (n / 1000) % 10 {
        0 => "",
        1 => "M",
        2 => "MM",
        3 => "MMM",
        4 => "MMMM",
        5 => "MMMMM",
        6 => "MMMMMM",
        7 => "MMMMMMM",
        8 => "MMMMMMMM",
        9 => "MMMMMMMMM",
        _ => unreachable!(),
    }
    .to_string()
}
