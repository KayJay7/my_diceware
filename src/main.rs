mod diceware;
mod utility;

fn main() {
    let rounds = diceware::parse_arguments(std::env::args());
    let dictionary = utility::get_dictionary();

    // If the integrity check fails terminate the program
    if !utility::check_integrity(&dictionary) {
        eprintln!(
            "The internal dictionary has been altered!\n\
            This program is unable to produce secure passwords!\n\
            Please replace the executable"
        );
        std::process::exit(1);
    }

    // Generate random dictionary indexes
    let randoms = diceware::generate_randoms(rounds);

    // Print generated passwords
    diceware::print_passwords(dictionary, randoms);
}
