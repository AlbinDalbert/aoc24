use std::collections::HashMap;
use crate::Problem;
use crate::get_input;
use crate::ToGrid;

pub struct DayEight{}

impl Problem for DayEight {

    fn part_one(&self) -> String {
        let antennas: Scan = read_input(get_input(8));

        let mut solution = 0;
        

        solution.to_string()
    }

    fn part_two(&self) -> String {
        get_input(0);
        
        let mut solution = 0;


        solution.to_string()
    }
}

struct Position {
    x: i32,
    y: i32,
}

struct Antenna {
    pos: Position,
    frequency: char,
}

struct Scan {
    antennas: HashMap<char, Vec<Antenna>>,
    bounds: Position,
}

fn read_input(input: String) -> Scan {
     
    let grid = input.to_grid();
    
    let mut frequency_map: HashMap<char, Vec<Antenna>> = HashMap::new();
    
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '.' {continue;} 
            frequency_map.entry(grid[y][x])
                .or_insert(Vec::new())
                .push(
                    Antenna {
                        pos: Position {x: x as i32, y: y as i32},
                        frequency: grid[y][x],
                    }
                );   
        }
    }

    Scan {
        antennas: frequency_map,
        bounds: Position{x: grid[0].len() as i32, y: grid.len() as i32},
    }
}