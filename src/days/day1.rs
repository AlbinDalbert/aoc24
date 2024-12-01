use std::collections::HashMap;
use std::ops::Index;

use crate::Problem;
use crate::get_input;

pub struct DayOne{}

impl Problem for DayOne {
    fn part_one(&self) -> String {
        let input = get_input(1);

        let (mut list_1, mut list_2): (Vec<i32>, Vec<i32>) = input_to_tuples(input);
        
        list_1.sort();
        list_2.sort();

        let mut diff_list: Vec<u32> = Vec::new();

        for i in 0..list_1.len() {
            let dif = list_1.index(i).abs_diff(*list_2.index(i));
            diff_list.push(dif);
        }
        
        let total: u32 = diff_list.into_iter().sum();
        total.to_string()
    }

    fn part_two(&self) -> String {
        let input = get_input(1);

        let (mut list_1, mut list_2): (Vec<i32>, Vec<i32>) = input_to_tuples(input);
        
        list_1.sort();
        list_2.sort();

        let mut sim_list: Vec<u32> = Vec::new();

        let mut frequency_table: HashMap<i32, i32> = HashMap::new();

        for num in list_2 {
            *frequency_table.entry(num).or_insert(0) += 1;
        }

        for i in list_1 {
            let sim = frequency_table.get(&i).unwrap_or(&0);
            sim_list.push((i * sim).try_into().unwrap());
        }
        
        let total: u32 = sim_list.into_iter().sum();
        total.to_string()
    }
}

fn input_to_tuples(input: String) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace()
                .map(|numb| numb.parse::<i32>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        }).unzip()
}