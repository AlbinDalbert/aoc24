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
        get_input(4)
        // todo!()
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

        if word[step] != input[new_x as usize][new_y as usize] {
            return 0;
        }
    }    
    1
}