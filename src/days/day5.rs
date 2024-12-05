use std::collections::HashMap;
use std::collections::VecDeque;
use std::result;

use crate::Problem;
use crate::get_input;

/// the swap when missmatch occures causes the problem probably. caues other rules to be borken.
/// 

pub struct DayFive{}

impl Problem for DayFive {
    fn part_one(&self) -> String {
        let mut solution = 0;
        
        let input_str = get_input(5);

        let lines: Vec<&str> = input_str.lines().collect();

        let input1 = &lines[0..1177];
        let input2 = &lines[1178..]; // Start after the empty line

        let rules = set_rules(input1);
        println!("num of rules: {}", rules.forward.len());

        let mut updates = input2.into_iter()
            .map(|li| li.split(',')
                .map(|s| {
                    s.parse::<i32>().unwrap()
                }).collect::<VecDeque<i32>>())
            .collect::<Vec<VecDeque<i32>>>();

        let mut changed: Vec<usize> = Vec::new();
        for _ in 0..100 {

            for upd in 0..updates.len() {
                for page in 0..updates[upd].len() {
                    if let Some((value, boll)) = rules.look_up(updates[upd][page]) 
                    {   
                        for val in value {

                            if updates[upd].contains(val) {
                                let val_index = updates[upd].iter().position(|&x| x == *val).unwrap();
                                if page > val_index && boll {
                                    
                                    // println!("Swap \t{} to the front in \t {:?}", updates[upd][page], updates[upd]);

                                    updates[upd].rotate_left(page);

                                    if !changed.contains(&upd) {
                                        changed.push(upd);
                                    }
                                    break;
                                }
                                if page < val_index && !boll {
                                    // println!("Swap \t{} to the back in \t {:?}", updates[upd][page], updates[upd]);
                                    
                                    updates[upd].rotate_right(page);
                                    
                                    if !changed.contains(&upd) {
                                        changed.push(upd);
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        // println!("{updates:?}");
        solution = 0;
        for row in changed {
            println!("{:?}", updates[row]);
            solution += updates[row][updates[row].len() / 2];
        }

        solution.to_string()
    }

    fn part_two(&self) -> String {
        get_input(5);
        
        let mut solution = 0;


        solution.to_string()
    }
}

struct RuleMap {
    forward: HashMap<i32, Vec<i32>>,
    backward: HashMap<i32, Vec<i32>>,
}

impl RuleMap {
    fn new() -> Self {
        RuleMap {
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }

    fn add_new_rule(&mut self, x: i32, y: i32) {
        self.forward.entry(x).or_default().push(y);
        self.backward.entry(y).or_default().push(x);
    }

    fn look_up(&self, val: i32) -> Option<(&Vec<i32>, bool)> {
        if let Some(right) = self.forward.get(&val) {
            Some((right, true))
        } else if let Some(left) = self.backward.get(&val) {
            Some((left, false))
        } else {
            None
        }
    }
}

fn set_rules(raw: &[&str]) -> RuleMap {
    let mut rules = RuleMap::new();
    let result = raw.into_iter()
        .filter_map(|s| {
            s.split_once("|")
        })
        .for_each(|(left, right)| {
            rules.add_new_rule(left.parse().unwrap(), right.parse().unwrap());
        });
    
    rules
}