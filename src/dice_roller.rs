extern crate rand;

use crate::input_error::*;
use rand::Rng;
use std::error;
use std::io;

pub const MIN_NUMBER_OF_DICES: u64 = 1;
pub const MAX_NUMBER_OF_DICES: u64 = 100;
pub const MIN_NUMBER_OF_SIDES: u64 = 2;
pub const MAX_NUMBER_OF_SIDES: u64 = 100;

pub struct DiceCombination {
    number_of_dices: u64,
    number_of_sides: u64,
}

impl DiceCombination {
    pub fn number_of_dices(&self) -> u64 {
        self.number_of_dices
    }

    pub fn number_of_sides(&self) -> u64 {
        self.number_of_sides
    }
}

fn get_input_string() -> Result<String, io::Error> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)?;
    Ok(input_string)
}

fn parse_and_check_if_in_range(string: &str, min: u64, max: u64) -> Result<u64, InputError> {
    let number_of_dices: u64 = match string.trim().parse() {
        Ok(number_of_dices) => number_of_dices,
        Err(_) => return Err(InputError::WrongFormat),
    };
    if !is_in_range(number_of_dices, min, max) {
        Err(InputError::WrongFormat)
    } else {
        Ok(number_of_dices)
    }
}

fn is_in_range(value: u64, min: u64, max: u64) -> bool {
    value >= min && value <= max
}

fn parse_input_string(input_string: String) -> Result<(u64, u64), InputError> {
    // Check if input string contains a d
    let trimed_string = input_string.trim();
    if !trimed_string.contains('d') {
        return Err(InputError::WrongFormat);
    }

    // Split the string and check if we have 2 strings
    let vec_of_string: Vec<&str> = trimed_string.split('d').collect();
    if vec_of_string.len() != 2 {
        return Err(InputError::WrongFormat);
    }

    // Parse the strings to u64 and check their value
    let number_of_dices = parse_and_check_if_in_range(
        vec_of_string.first().ok_or(InputError::WrongFormat)?,
        MIN_NUMBER_OF_DICES,
        MAX_NUMBER_OF_DICES,
    )?;

    let number_of_sides = parse_and_check_if_in_range(
        vec_of_string.last().ok_or(InputError::WrongFormat)?,
        MIN_NUMBER_OF_SIDES,
        MAX_NUMBER_OF_SIDES,
    )?;

    Ok((number_of_dices, number_of_sides))
}

pub fn parse_input_and_make_dice_combination() -> Result<DiceCombination, Box<dyn error::Error>> {
    println!("Enter your dice");
    let input_string = get_input_string()?;
    let number_of_dices_and_sides = parse_input_string(input_string)?;
    Ok(DiceCombination {
        number_of_dices: number_of_dices_and_sides.0,
        number_of_sides: number_of_dices_and_sides.1,
    })
}

pub fn generate_result(dice_combination: DiceCombination) -> u64 {
    (0..dice_combination.number_of_dices)
        .map(|_| rand::thread_rng().gen_range(1, dice_combination.number_of_sides + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_in_range_test() {
        assert_eq!(true, is_in_range(12, 1, 16));
        assert_eq!(false, is_in_range(16, 1, 12));
    }
}
