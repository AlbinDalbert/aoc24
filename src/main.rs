use std::{any::Any, env, process::exit, time::Instant, usize};

use days::*;
mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args.get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let part = args.get(2)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(-1);

    let prob_wrp = day_to_problem(day);

    if prob_wrp.is_none() {
        println!("Day don't exists");
        exit(-1);
    }
    
    let prob: Box<dyn Problem> = prob_wrp.unwrap();
    
    if let Some(_) = prob.as_day_zero() {
        println!("Solution not yet implemented");
        exit(-1);
    }

    let start = Instant::now();
    match part {
        1 => println!("{}", prob.part_one()),
        2 => println!("{}", prob.part_two()),
        _ => {
            println!("----\npart 1:\n{}\n----",prob.part_one());
            println!("part 2:\n{}\n----",prob.part_two());
        }
    }
    let speed = start.elapsed();
    println!("time: {:?}", speed);
}

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne{})),
        2 => Some(Box::new(DayTwo{})),
        3 => Some(Box::new(DayThree{})),
        4 => Some(Box::new(DayFour{})),
        5 => Some(Box::new(DayFive{})),
        6 => Some(Box::new(DaySix{})),
        1..26 => Some(Box::new(DayZero{})),
        _ => None
    }
}


pub trait Problem: Any {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
    fn as_day_zero(&self) -> Option<&DayZero> {
        None
    }
}

fn get_input(day: usize) -> String {
    std::fs::read_to_string(format!("input/day{}.md", day))
        .expect("Could not read input file")
}