use crate::Problem;
use crate::get_input;

pub struct DayFour{}

impl Problem for DayFour {
    fn part_one(&self) -> String {
        let input_str = get_input(4);
        let input = input_str
            .lines()
                .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        
        let mut solution = 0;
        
        let word: Vec<char> = "XMAS".to_string().chars().collect();

        for x in 0..input.len() {
            for y in 0..input[x].len() {
                if input[x][y] == word[0] {
                    solution += search_area(x, y, &word, &input)
                }
            }
        }

        solution.to_string()
    }

    fn part_two(&self) -> String {
        let input_str = get_input(4);
        let input = input_str
            .lines()
                .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        
        let mut solution = 0;

        let word1: Vec<char> = "AM".to_string().chars().collect();
        let word2: Vec<char> = "AS".to_string().chars().collect();

        for x in 0..input.len() {
            for y in 0..input[x].len() {
                if input[x][y] == word1[0] {
                    solution += is_x_mas(x, y, &word1, &word2, &input);
                }
            }
        }

        solution.to_string()
    }
}


fn search_area(x: usize, y: usize, word: &Vec<char>, input: &Vec<Vec<char>>) -> u32 {
    let mut sol = 0;
    let directions = [
        (-1, -1), (0, -1), (1, -1),  
        (-1,  0),          (1,  0),  
        (-1,  1), (0,  1), (1,  1)   
    ];

    for dir in directions {
        sol += search_dir(x as i32, y as i32, &word, input, dir);
    }

    sol
}

fn search_dir(x: i32, y: i32, word: &Vec<char>, input: &Vec<Vec<char>>, dir: (i32,i32)) -> u32 {
    let (x_dir, y_dir) = dir;
    
    if x + (x_dir * (word.len()-1) as i32) >= input[0].len() as i32 || 
        x + (x_dir * (word.len()-1) as i32) < 0 ||
        y + (y_dir * (word.len()-1) as i32) >= input.len() as i32 || 
        y + (y_dir * (word.len()-1) as i32) < 0 {
            return 0;
    }
    
    for step in 0..word.len() {
        let new_x = x + (x_dir * step as i32);
        let new_y = y + (y_dir * step as i32);
        
        if new_x < 0 || new_y < 0 || 
        new_x >= input.len() as i32 || 
        new_y >= input[0].len() as i32 {
            return 0;
        }
        
        if word[step] != input[new_x as usize][new_y as usize] {
            return 0;
        }
    }    
    
    1
}

fn is_x_mas(x: usize, y: usize, word1: &Vec<char>,word2: &Vec<char>, input: &Vec<Vec<char>>) -> u32 {
    let x= x as i32;
    let y = y as i32;
    let directions = [
        (-1, -1), (1, -1),  
        (-1,  1), (1,  1)   
    ];

    for dir in directions {

        if check_diagnonal_mas(x, y, word1, word2, input, dir) {
            let (x_dir,y_dir) = dir;
            
            if check_diagnonal_mas(x, y, word1, word2, input, (-x_dir, y_dir)) {
                return 1;
            }
            return 0;
        }
    }

    0
}

fn check_diagnonal_mas(x: i32, y: i32, word1: &Vec<char>, word2: &Vec<char>, input: &Vec<Vec<char>>, dir: (i32,i32)) -> bool {
    if search_dir(x, y, word1, input, dir) == 1 {
        let (x_dir,y_dir) = dir;
        if search_dir(x, y, word2, input, (-x_dir, -y_dir)) == 1 {
            return true;
        }
    }

    let (x_dir,y_dir) = dir;
    let dir = (-x_dir, -y_dir);

    if search_dir(x, y, word1, input, dir) == 1 {
        let (x_dir,y_dir) = dir;
        if search_dir(x, y, word2, input, (-x_dir, -y_dir)) == 1 {
            return true;
        }
    }

    false
}