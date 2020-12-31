use rand_distr::Distribution;
use rand_distr::Uniform;
use std::env::Args;

pub fn generate_randoms(rounds: u32) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0..7776);
    let mut randoms = Vec::new();

    // Produces N random dictionary indexes
    for _i in 0..rounds {
        randoms.push(dist.sample(&mut rng));
    }

    randoms
}

pub fn parse_arguments(args: Args) -> u32 {
    let args: Vec<String> = args.collect();
    if args.len() == 1 {
        // If unspecified defaults to 4 rounds
        4
    } else if args.len() == 2 {
        // Otherwise use specified rounds count
        parse_rounds(&args[1])
    } else {
        // If more than 1 argument is found, display usage information
        eprintln!(
            "Usage:
    my_diceware [rounds]

Rounds is the (non negative) number of outputed words, default 4."
        );
        std::process::exit(1);
    }
}

fn parse_rounds(arg: &String) -> u32 {
    // If unable to parse arguments, display an error and exit
    match arg.parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid or out of range argument");
            std::process::exit(1);
        }
    }
}

pub fn print_passwords(dictionary: [&str; 7776], randoms: Vec<usize>) {
    // Print words separated by spaces
    print!("Passphrase:");
    for index in &randoms {
        // Indexes are borrowed for later use
        print!(" {}", dictionary[*index]);
    }
    print!("\n");

    // Print concatenated words in LeadingCamelCase
    print!("Password: ");
    for index in &randoms {
        // Safely uppercasing the first letter in a sting is convoluted,
        // take your time to read through
        let mut temp = dictionary[*index].chars();
        print!(
            "{}",
            match temp.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + temp.as_str(),
            }
        );
    }
    print!("\n");
}
