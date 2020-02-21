mod dice_roller;
mod input_error;

fn help() {
    println!("Input format should be NdM where N is a integer between {} and {} and M is a integer between {} and {}",
    dice_roller::MIN_NUMBER_OF_DICES, dice_roller::MAX_NUMBER_OF_DICES, dice_roller::MIN_NUMBER_OF_SIDES, dice_roller::MAX_NUMBER_OF_SIDES);
}

fn main() {
    loop {
        let dice_combination = match dice_roller::parse_input_and_make_dice_combination() {
            Ok(dice_combination) => dice_combination,
            Err(e) => {
                println!("Error: {}", e);
                help();
                continue;
            }
        };

        println!("Number of dices = {}", dice_combination.number_of_dices());
        println!("Number of sides = {}", dice_combination.number_of_sides());

        let result = dice_roller::generate_result(dice_combination);
        println!("Result = {}", result);
        break;
    }
}
