use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 4 {
        println!("Usage: {} <input_file> <output_file> <operation>", args[0]);
        println!("Operations: uppercase, lowercase, reverse");
        return;
    }

    // Extract command-line arguments
    let input_file = &args[1];
    let output_file = &args[2];
    let operation = &args[3];

    // Read the content of the input file
    let input_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading input file '{}'", input_file);
            return;
        }
    };

    // Perform the specified text manipulation operation
    let result = match operation.as_str() {
        "uppercase" => input_content.to_uppercase(),
        "lowercase" => input_content.to_lowercase(),
        "reverse" => input_content.chars().rev().collect(),
        _ => {
            eprintln!("Invalid operation '{}'", operation);
            return;
        }
    };

    // Write the result to the output file
    if let Err(err) = fs::write(output_file, result) {
        eprintln!("Error writing to output file '{}': {}", output_file, err);
    } else {
        // Print a success message
        println!("Operation completed successfully. Result written to '{}'", output_file);
    }
}