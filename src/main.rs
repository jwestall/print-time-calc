mod ptime;

use std::io;
use crate::ptime::ptime::PTime;

fn main() {
    let start_time = PTime::get_ptime("start".to_string());
    let duration = PTime::get_ptime("print".to_string());
    let end_time = compare_time(start_time, duration);

    println!("Your print will finish at {}:{}.", end_time[0], end_time[1]);
}

fn get_time(time_name: String) -> [u32; 2] {
    let mut input = String::new();
    let mut output: [u32; 2] = [0, 0];
    let mut i = 0;

    println!("Enter the {} time as HH:MM below:", time_name);
    io::stdin().read_line(&mut input).expect("Error reading from stdin!");

    for s in input.split(":") {
        output[i] = s.trim().parse().expect("Error parsing input to u32!");
        i += 1;
    }

    output
}

fn compare_time(start: [u32; 2], duration: [u32; 2]) -> [u32; 2] {
    let mut end: [u32; 2] = [0, 0];

    end[0] = start[0] + duration[0];
    end[1] = start[1] + duration[1];

    if end[1] > 60 {
        end[0] += 1;
        end[1] -= 60;
    }

    if end[0] > 24 {
        end[0] -= 24;
    } else if end[0] > 12 {
        end[0] -= 12;
    }

    end
}