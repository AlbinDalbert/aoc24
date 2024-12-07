use std::fmt;
use std::collections::VecDeque;

use crate::Problem;
use crate::get_input;

pub struct DaySeven{}

impl Problem for DaySeven {
    fn part_one(&self) -> String {

        let equations = read_input(get_input(7));

        let mut solution = 0;
        
        for equa in &equations {
            if equa.evaluat_mul_add() {
                solution += equa.result;
            }
        }

        solution.to_string()
    }

    fn part_two(&self) -> String {

        let equations = read_input(get_input(7));

        let mut solution = 0;
        
        for mut equa in equations {
            if equa.evaluat_mul_add_concat() {
                //println!("{}", equa.to_string());
                solution += equa.result;
            }
        }

        solution.to_string()
    }
}

struct Equation {
    result: u64,
    numbers: Vec<u64>,
    operands: Vec<Operands>,
}

#[derive(Clone, Copy, Debug)]
enum Operands{
    Addition,
    Multiplication,
    Concatination,
}

impl Equation {
    fn new(mut input: VecDeque<u64>) -> Equation {

        let mut eq = Equation {
            result: input.pop_front().unwrap(),
            numbers: Vec::new(),
            operands: Vec::new(),
        };
        while let Some(num) = input.pop_front() {
            eq.numbers.push(num);
        }

        eq
    }

    fn evaluat_mul_add(&self) -> bool {
        self.try_mul_add_combinations(&self.numbers, 0)
    }

    fn evaluat_mul_add_concat(&mut self) -> bool {
        let numbers = self.numbers.clone();
        self.try_mul_add_concat_combinations(&numbers[1..], self.numbers[0], Vec::new())
    }

    fn try_mul_add_combinations(&self, numbers: &[u64], current_value: u64) -> bool {
        if numbers.is_empty() {
            return current_value == self.result;
        }

        if current_value > self.result {
            return false;
        }

        if numbers.len() == 1 {
            return current_value + numbers[0] == self.result || 
                   current_value * numbers[0] == self.result;
        }

        if self.try_mul_add_combinations(&numbers[1..], current_value + numbers[0]) {
            return true;
        }

        self.try_mul_add_combinations(&numbers[1..], current_value * numbers[0])
    }

    fn try_mul_add_concat_combinations(&mut self, numbers: &[u64], current_value: u64, mut current_operands: Vec::<Operands>) -> bool {

        if numbers.is_empty() {
            
            if current_value == self.result {
                self.operands = current_operands.clone();
                return true;
            }
            return false;
        }

        if current_value > self.result {
            return false;
        }

        if numbers.len() == 1 {
            let mut try_ops = current_operands.clone();
            try_ops.push(Operands::Addition);
            
            if current_value + numbers[0] == self.result {
                self.operands = try_ops;
                return true;
            }
            let mut try_ops = current_operands.clone();
            try_ops.push(Operands::Multiplication);

            if current_value * numbers[0] == self.result {
                self.operands = try_ops;
                return true;
            };

            let mut try_ops = current_operands.clone();
            try_ops.push(Operands::Concatination);
            let concat = format!("{}{}", current_value, numbers[0])
                .parse::<u64>()
                .unwrap();

            if concat == self.result {
                self.operands = try_ops;
                return true;
            }
            return false;
                   
        }
        
        let mut try_ops = current_operands.clone();
        try_ops.push(Operands::Addition);
        if self.try_mul_add_concat_combinations(&numbers[1..], current_value + numbers[0], try_ops.clone()) {
            current_operands.push(Operands::Addition);
            return true;
        }

        let mut try_ops = current_operands.clone();
        try_ops.push(Operands::Multiplication);
        if self.try_mul_add_concat_combinations(&numbers[1..], current_value * numbers[0], try_ops.clone()) {
            current_operands.push(Operands::Multiplication);
            return true;
        }

        let concat = format!("{}{}", current_value, numbers[0])
            .parse::<u64>()
            .unwrap();

        let mut try_ops = current_operands.clone();
        try_ops.push(Operands::Concatination);
        if self.try_mul_add_concat_combinations(&numbers[1..], concat, try_ops.clone()) {
            current_operands.push(Operands::Concatination);
            return true;
        }
        return false;
    }

}


impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.result)?;
        write!(f, "{}", self.numbers[0])?;
        for i in 1..self.numbers.len() {
            let op = match self.operands[i-1] {
                Operands::Addition => "+",
                Operands::Multiplication => "*",
                Operands::Concatination => "_",
            };
            write!(f, " {} {}", op, self.numbers[i])?;
        }
        Ok(())
    }
}

fn read_input(str: String) -> Vec<Equation> {
    let input_str = get_input(7);
    let lines: Vec<&str> = input_str.lines().collect();
    let equations_numbers: Vec<VecDeque<u64>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.trim_end_matches(':')) 
                .filter_map(|num| num.parse().ok())
                .collect()
        })
        .collect();
    
    let mut equations: Vec<Equation> = Vec::new();
    for numbers in equations_numbers {
        equations.push(Equation::new(numbers));
    }
    equations
}