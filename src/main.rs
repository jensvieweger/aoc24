mod days;
mod utils;

use crate::days::day01::*;
use crate::days::day02::*;
use crate::days::day03::*;
use crate::days::day03_2::*;
use crate::utils::*;

use std::env;

fn day01() {
    println!("{:?}", get_day01(&read_lines()).unwrap());
    println!("{:?}", get_day01_part2(&read_lines()).unwrap());
}

fn day02() {
    println!("{:?}", get_day02(&read_lines()).unwrap());
    println!("{:?}", get_day02_part2(&read_lines()).unwrap());
}

fn day03() {
    println!("{:?}", get_day03(&read_lines()).unwrap());
}

fn day03_2() {
    println!("{:?}", get_day03_part2(&read_lines()).unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "01" => day01(),
        "02" => day02(),
        "03" => day03(),
        "03_2" => day03_2(),
        _ => return
    };
}
