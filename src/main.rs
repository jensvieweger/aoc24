mod days;
mod utils;

use crate::days::day01::*;
use crate::utils::*;

use std::env;

fn day01() {
    let cal_vals = extract_calvals(&read_lines());
    let sum:u32 = cal_vals.iter().map(|&b| b as u32).sum();
    println!("s{:?}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "01" => day01(),
        _ => return
    };
}
