use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let maximum_length = 6;
    let actual_roman_numbers = original_roman_numbers(maximum_length);
    let claimed_valid_strings = claimed_valid_roman_numbers(maximum_length);

    println!("Non-claimed but valid: {:?}", &actual_roman_numbers - &claimed_valid_strings);
    println!("Invalid but claimed: {:?}", &claimed_valid_strings - &actual_roman_numbers);
}

fn original_roman_numbers(maximum_length: usize) -> HashSet<String> {
    (1..4000)
        .map(roman_from_decimal)
        .filter(|r| r.len() <= maximum_length)
        .collect()
}

fn claimed_valid_roman_numbers(maximum_length: usize) -> HashSet<String> {
    (0..maximum_length)
        .map(|_| {
            ["", "I", "V", "X", "L", "C", "D", "M"]
                .iter()
                .map(|s| s.to_string())
        })
        .multi_cartesian_product()
        .map(|vector| vector.iter().join(""))
        .filter(valid_roman_number)
        .filter(|s| s.len() > 0)
        .collect()
}

fn valid_roman_number(v: &String) -> bool {
    ![
        "LD", "VX", "IC", "DD", "VV", "VL", "IM", "DM", "LC", "VD", "IL", "XD", "VM", "XM", "LL",
        "VC", "LM", "ID", "DCD", "XLX", "XXL", "IXC", "CMD", "VIV", "CCM", "CMC", "CMM", "XXC",
        "IXX", "DCM", "CDC", "IXI", "XCC", "LXC", "VIX", "XCM", "XCL", "CCD", "IXL", "IIX", "LXL",
        "IXV", "IVI", "XCX", "IIV", "XCD", "CCCC", "XXXX", "IIII", "MMMM",
    ]
    .iter()
    .any(|s| v.contains(&s.to_string()))
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
