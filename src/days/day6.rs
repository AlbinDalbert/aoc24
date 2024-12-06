use crate::Problem;
use crate::get_input;

pub struct DaySix{}

impl Problem for DaySix {

    fn part_one(&self) -> String {

        let input_str = get_input(6);
        let lines: Vec<&str> = input_str.lines().collect();

        let mut map = get_map_from_lines(lines);
        let mut visits = 0;
        while map.in_bounds {
            visits += 1;
            match map.get_state_in_front() {
                None => map.in_bounds = false,
                Some(MapState::Block) => map.direction.rotate_right(),
                _ => map.step_forward(),
             }
        }

        let mut solution = 0;

        solution = map.tiles.iter()
            .flatten()
            .filter(|&state| matches!(state, MapState::Visited(_)))
            .count();

        solution.to_string()
    }

    fn part_two(&self) -> String {
        let input_str = get_input(6);
        let lines: Vec<&str> = input_str.lines().collect();

        let map = get_map_from_lines(lines);

        let solution = map.find_loops();

        solution.to_string()
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_left(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<MapState>>,
    position: (usize, usize),
    direction: Direction,
    in_bounds: bool,
}

impl Map {
    // None if pos in front is out of bounds
    fn get_state_in_front(&self) -> Option<MapState> {
        let (x, y) = self.position;
        let (next_x, next_y) = match self.direction {
            Direction::North => (x, y.checked_sub(1)?),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x.checked_sub(1)?, y),
        };
        
        self.tiles.get(next_y)?.get(next_x).copied()
    }

    fn as_string(&self) -> String {
        let mut result = String::new();
        
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, state) in row.iter().enumerate() {
                // Add character based on current position and state
                if (x, y) == self.position {
                    result.push(match self.direction {
                        Direction::North => '^',
                        Direction::South => 'v',
                        Direction::East => '>',
                        Direction::West => '<',
                    });
                } else {
                    result.push(match state {
                        MapState::Free => '.',
                        MapState::Visited(_) => 'X',
                        MapState::Block => '#',
                    });
                }
            }
            result.push('\n');
        }
        
        result
    
    }

    //assums validity
    fn step_forward(&mut self) {
        let (x, y) = self.position;
        let (next_x, next_y) = match self.direction {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        };
        self.tiles[next_y][next_x] = MapState::Visited(self.direction);
        self.position = (next_x, next_y);
    }

    fn find_loops(&self) -> i32 {
        let mut result = 0;
        
        // Try each free position
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if self.tiles[y][x] == MapState::Free {
                    let mut mod_map = self.clone();
                    mod_map.tiles[y][x] = MapState::Block;

                    while mod_map.in_bounds {
                        match mod_map.get_state_in_front() {
                            Some(MapState::Visited(dir)) if dir == mod_map.direction => {
                                result += 1;
                                break;
                            },
                            None => mod_map.in_bounds = false,
                            Some(MapState::Block) => mod_map.direction.rotate_right(),
                            _ => mod_map.step_forward(),
                        }
                    }
                }
            }
        }
        
        result
    }

}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
enum MapState {
    Free,
    Visited(Direction),
    Block,
}

fn get_map_from_lines(lines: Vec<&str>) -> Map {
    let mut start_pos = (0, 0);
    let tiles = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => MapState::Free,
                    '#' => MapState::Block,
                    '^' => {
                        start_pos = (x, y);
                        MapState::Visited(Direction::North)
                    }
                    _ => panic!("Invalid character in map"),
                })
                .collect()
        })
        .collect();

    Map {
        tiles,
        position: start_pos,
        direction: Direction::North,
        in_bounds: true,
    }
}