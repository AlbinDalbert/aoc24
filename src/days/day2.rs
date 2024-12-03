use crate::Problem;
use crate::get_input;

pub struct DayTwo{}

impl Problem for DayTwo {

    fn part_one(&self) -> String {
        
        let input = get_input(2);
        let mut solution = 0;

        let reports: Vec<Vec<i32>> = 
        input.lines()
            .map(|line| line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec::<i32>>()
        ).collect();

        let max_jump = 3;
        for report in reports {
            
            if report.windows(2).all(|pair| {
                let dif = pair[1] - pair[0];
                dif > 0 && dif <= max_jump
            }) {
                solution += 1;
            } else if report.windows(2).all(|pair| {
                let dif = pair[0] - pair[1];
                dif > 0 && dif <= max_jump
            }){
                solution += 1;
            };
        }

       solution.to_string()
    }


    fn part_two(&self) -> String {
        let input = get_input(2);
        let mut solution = 0;

        let reports: Vec<Vec<i32>> = 
        input.lines()
            .map(|line| line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec::<i32>>()
        ).collect();
        
        let max_jump = 3;
        for report in reports {

            if check_for_increasing_pattern(&report, &max_jump) {
                solution += 1;
                continue;
            }

            if check_for_decrease_pattern(&report, &max_jump) {
                solution += 1;
            }   

        }

       solution.to_string()
    }
}

fn check_for_increasing_pattern(report: &Vec<i32>, max_jump: &i32) -> bool{
    if report.windows(2).all(|pair| {
        let dif = pair[1] - pair[0];
        dif > 0 && dif <= *max_jump
    }) {
        return true;
    } 
    // checks if removing a entry resolves the problem
    for skip_idx in 0..report.len() {
        let modified: Vec<i32> = report.iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, &x)| x)
            .collect();

        if modified.windows(2).all(|pair| {
            let dif = pair[1] - pair[0];
            dif > 0 && dif <= *max_jump
        }) {
            return true;
        }
    }
    false
}

fn check_for_decrease_pattern(report: &Vec<i32>, max_jump: &i32) -> bool{
    if report.windows(2).all(|pair| {
        let dif = pair[0] - pair[1];
        dif > 0 && dif <= *max_jump
    }) {
        return true;
    } 
    // checks if removing a entry resolves the problem
    for skip_idx in 0..report.len() {
        let modified: Vec<i32> = report.iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, &x)| x)
            .collect();

        if modified.windows(2).all(|pair| {
            let dif = pair[0] - pair[1];
            dif > 0 && dif <= *max_jump
        }) {
            return true;
        }
    }
    false
}