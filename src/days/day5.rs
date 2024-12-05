use std::mem::swap;

use crate::Problem;
use crate::get_input;

pub struct DayFive{}

impl Problem for DayFive {

    fn part_one(&self) -> String {

        let input_str = get_input(5);

        let lines: Vec<&str> = input_str.lines().collect();

        let input1 = &lines[0..1177];
        let input2_str = &lines[1177..]; 

        let input2_wrp: Result<Vec<Vec<i32>>, std::num::ParseIntError> = input2_str
        .iter()
        .map(|s| {
            s.split(',')
                .map(|num| num.trim().parse::<i32>()) // Parse each number
                .collect() 
        })
        .collect();
        let input2: Vec<Vec<i32>> = input2_wrp.unwrap();
        
        let rules = parse_rules(input1);

        let mut valid_updates: Vec<Vec<i32>> = Vec::new();
        for row in input2 {
            if rules.check_row(row.clone()) {
                valid_updates.push(row);
            }
        }

        let mut solution = 0;
        
        for row in valid_updates {
            solution += row[row.len() / 2];
        }

        solution.to_string()
    }

    fn part_two(&self) -> String {
        let input_str = get_input(5);

        let lines: Vec<&str> = input_str.lines().collect();

        let input1 = &lines[0..1177];
        let input2_str = &lines[1177..];

        let input2_wrp: Result<Vec<Vec<i32>>, std::num::ParseIntError> = input2_str
        .iter()
        .map(|s| {
            s.split(',')
                .map(|num| num.trim().parse::<i32>()) // Parse each number
                .collect() 
        })
        .collect();
        let input2: Vec<Vec<i32>> = input2_wrp.unwrap();
        
        let rules = parse_rules(input1);

        let mut invalid_updates: Vec<Vec<i32>> = Vec::new();
        for row in input2 {
            if !rules.check_row(row.clone()) {
                invalid_updates.push(row);
            }
        }

        for _ in 0..100 {

            let mut swaps = 0;
            for line in 0..invalid_updates.len() {
                
                for index in 0..invalid_updates[line].len() {
                    let should_after_nums = rules.get_rules(invalid_updates[line][index]);
                    for rul in should_after_nums {
                        if let Some(after_index) = invalid_updates[line].iter().position(|&x| x == rul) {
                            
                            if after_index < index {

                                invalid_updates[line].swap(index, after_index);
                                swaps += 1;
                            }
                        }
                    }
                    
                }
                
            }
            if swaps == 0 {break};
        }
            
        let mut solution = 0;

        for row in invalid_updates {
            solution += row[row.len() / 2];
        }
        

        solution.to_string()
    }
}

struct Rule {
    before: i32,
    after: i32,
}

struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn new() -> Self {
        Rules { rules: Vec::new() }
    }

    fn add_rule(&mut self, before: i32, after: i32) {
        
        self.rules.push(Rule { before, after });
    }

    fn check_row(&self, row: Vec<i32>) -> bool {
        for rule in &self.rules {
            // Only check if both numbers exist in the row
            if let (Some(pos_before), Some(pos_after)) = (
                row.iter().position(|&x| x == rule.before),
                row.iter().position(|&x| x == rule.after)
            ) {
                if pos_after < pos_before {
                    return false;
                }
            }
        }
        true
    }

    fn get_rules(&self, key: i32) -> Vec<i32> {
        self.rules
        .iter()
        .filter(|rule| rule.before == key)
        .map(|rule| rule.after)
        .collect()
    }
}

fn parse_rules(input: &[&str]) -> Rules {
    let mut rules = Rules::new();
    for line in input {
        if let Some((before, after)) = line.split_once('|') {
            
            rules.add_rule(
                before.trim().parse().unwrap(),
                after.trim().parse().unwrap()
            );
        }
    }
    rules
}