use crate::Problem;
use crate::get_input;

pub struct DayZero{}

impl Problem for DayZero {
    fn as_day_zero(&self) -> Option<&super::DayZero> {
        Some(self)
    }

    fn part_one(&self) -> String {
        get_input(0);

        let mut solution = 0;
        

        solution.to_string()
    }

    fn part_two(&self) -> String {
        get_input(0);
        
        let mut solution = 0;


        solution.to_string()
    }
}