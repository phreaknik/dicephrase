#[macro_use]
extern crate clap;
extern crate crc;

use clap::{App, Arg};
use crc::crc32;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::vec::Vec;

const DICE_SIDES: usize = 6;

fn main() {
    let arg_matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("MNEMONIC_TYPE")
                .help("What type of mnemonic phrase are you generating? Eg. 'monero-english'")
                .takes_value(true)
                .required(true),
        ).arg(
            Arg::with_name("dictionary")
                .short("d")
                .long("dictionary")
                .value_name("DICT_FILE")
                .help("Path to dictionary file.")
                .takes_value(true),
        ).get_matches();

    // Which type of mnemonic phrase are we generating?
    match arg_matches.value_of("type").unwrap() {
        "monero-english" => {
            if let Some(dict_file) = arg_matches.value_of("dictionary") {
                generate_mnemonic_monero(&dict_file);
            } else {
                generate_mnemonic_monero("dictionaries/monero-english.txt");
            }
        }
        _ => {
            println!("error: unable to determine mnemonic dictionary to use.");
            process::exit(1);
        }
    }
}

fn generate_mnemonic_monero(dict_file: &str) -> () {
    const DICT_SIZE: usize = 1626;
    let mut word_indices: Vec<usize> = Vec::new();
    let mut trimmed_words = String::new();

    // Open the dictionary file for reading
    let f = File::open(dict_file).unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });

    // Build dictionary from each line of dictionary file
    let dictionary: Vec<String> = BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    assert_eq!(dictionary.len(), 1626);

    // Calculate minimum # of rolls to preserve the most entropy
    let mut rem = DICT_SIZE;
    let mut min_rolls = 0;
    loop {
        rem = rem / DICE_SIDES;

        if rem <= 0 {
            break;
        }

        min_rolls += 1;
    }

    // Get dice rolls from user and return dictionary word
    println!("Enter 'q' or 'quit' to exit");
    println!("Enter dice rolls (without spaces), to generate a seed words.");
    println!(
        "Use at least {} rolls to preserve {}% entropy.",
        min_rolls,
        100 * DICE_SIDES.pow(min_rolls as u32) / DICT_SIZE
    );

    // Get first 24 words
    let mut current_word = 1;
    loop {
        // Prompt user to enter dice rolls
        let mut rolls: Vec<usize> = Vec::new();
        print!("({}/24)\t", current_word);
        if let Ok(input) = prompt_user("") {
            match &input as &str {
                // Detect quit command
                "q" | "Q" => process::exit(0),

                // Convert input to vector of numbers
                _ => {
                    rolls = input
                        .chars()
                        .map(|c| c.to_string().parse().expect("not a number"))
                        .collect()
                }
            }
        } else {
            println!("error: invalid input");
            continue;
        }

        // Loop for each roll to calculate large number
        let mut num = 0;
        let mut count = 1;
        for roll in rolls {
            // Calculate scale factor for this roll
            let scale_factor = DICT_SIZE / DICE_SIDES.pow(count);

            // Break if we have seen enough rolls
            if scale_factor <= 0 {
                break;
            }

            // Check this is a valid roll
            if roll > DICE_SIDES || roll < 1 {
                println!(
                    "error: invalid die roll: {}. Rolls must be between between 1-{}",
                    roll, DICE_SIDES
                );
                break;
            }

            // Calculate next part of number
            num += (roll - 1) * scale_factor;

            // Increment counter
            count += 1;
        }

        // Make sure all 4 rolls were summed
        if count < 5 {
            // Skip this word, since less than 4 dice were used
            println!(
                "error: roll at least {} valid dice to preserve entropy",
                min_rolls
            );
            continue;
        }

        // Look up dictionary word and add to phrase
        word_indices.push(num);
        trimmed_words.push_str(&dictionary[num as usize][0..3]);

        // Check if we have 24 words yet
        current_word += 1;
        if current_word > 24 {
            break;
        }
    }

    // Calculate checksum word
    let checksum = (crc32::checksum_ieee(trimmed_words.as_bytes()) as usize) % word_indices.len();
    word_indices.push(checksum);

    // Print phrase
    let mut idx = 1;
    for w in word_indices {
        println!("{}:\t{}", idx, dictionary[w as usize]);
        idx += 1;
    }
}

fn prompt_user(msg: &str) -> (Result<String, String>) {
    // Print message
    print!("{}", msg);
    io::stdout().flush().unwrap();

    // Read input
    let mut s = String::new();
    if let Err(error) = io::stdin().read_line(&mut s) {
        return Err(error.to_string());
    }

    // Strip off newline or carriage return characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    // Make sure input is non-empty
    if s.len() == 0 {
        return Err("No input detected.".to_string());
    } else {
        return Ok(s);
    }
}
