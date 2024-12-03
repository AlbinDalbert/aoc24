use std::iter::Peekable;
use std::str::Chars;

use crate::Problem;
use crate::get_input;
use regex::Regex;

pub struct DayThree{}

impl Problem for DayThree {
    fn part_one(&self) -> String {
        let input = get_input(3);
        let mut solution: i32 = 0;
        
        let regulator = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        for capture in regulator.captures_iter(&input){
            solution += &capture[1].parse::<i32>().unwrap() * &capture[2].parse::<i32>().unwrap();
        }

        solution.to_string()
    }

    fn part_two(&self) -> String {
        let input_str = get_input(3);
        let mut input = input_str.chars().peekable();
        let mut solution = 0;
        let mut active = true;

        while let Some(ch) = input.next(){
            match ch {
                // parse the dos and don'ts
                'd' => {
                    match match_d(&mut input) {
                        Some(val) => {
                            active = val
                        },
                        None => continue,
                    };
                },
                // parse the muls
                'm' => {
                    if active {
                        
                        match match_m(&mut input) {
                            Some(val) => {
                                
                                solution += val
                            },
                            None => continue,
                        }
                    }
                }
                _ => continue
            }
        }
        

        solution.to_string()
    }
}

fn match_d(chars: &mut Peekable<Chars>) -> Option<bool> {
    let mut temp_chars = chars.clone();
    if check_next_sequence(&mut temp_chars, "o()"){
        for _ in 0..3 {
            chars.next();
        }
        return Some(true);
    }
    let mut temp_chars = chars.clone();
    if check_next_sequence(&mut temp_chars, "on't()"){
        for _ in 0..6 {
            chars.next();
        }
        return Some(false);
    }
    return None;
}

fn match_m(chars: &mut Peekable<Chars>) -> Option<i32> {
    let mut temp_chars = chars.clone();
    
   if !check_next_sequence(&mut temp_chars, "ul("){
       return None;
   }
    
    let x = match check_for_number(&mut temp_chars) {
        Some(val) => {
            val
        },
        None => return None,
    };
    if temp_chars.peek() == Some(&',') {
        temp_chars.next();
    } else {
        return None;
    }
    let y = match check_for_number(&mut temp_chars) {
        Some(val) => {
            val
        },
        None => return None,
    };
    if temp_chars.peek() == Some(&')') {
        chars.next();
    } else {
        return None;
    }
    
    Some((x*y).try_into().unwrap())
}

fn check_next_sequence(chars: &mut Peekable<Chars>, expected: &str) -> bool {
    let mut expct = expected.chars();

    while let Some(expct_char) = expct.next() {
        //println!("{:?}, {:?}", chars.peek(), expct_char);

        if chars.peek().map_or(false,|&chr| chr == expct_char){
            chars.next();
        } else {
            return false;
        }
    }

    true
}

fn check_for_number(chars: &mut Peekable<Chars>) -> Option<u32> {
    let mut number = String::new();
    let mut count = 0;

    // println!("{}", prev_len);
    
    
    while count < 3 {
        if let Some(&ch) = chars.peek() {
            // println!("{}", ch);
            if ch.is_ascii_digit() {
                number.push(ch);
                chars.next(); // Consume the digit
                count += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    if !number.is_empty() {
        //println!("number is not empty: {}", number);
        number.parse().ok()
    } else {
        None
    }
}