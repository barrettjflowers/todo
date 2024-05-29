use clap::{Parser, Subcommand};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

// CLI todo list.
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A todo list CLI tool.", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ls,
    Create { content: String },
    Rm,
}

fn main() {
    let cli = Cli::parse();
    let file_name = "todo_list.txt";
    if !Path::new(file_name).exists() {
        match File::create(file_name) {
            Ok(file) => {
                println!("Created new todo file: {:?}", file);
            }
            Err(e) => {
                println!("Failed to create todo file: '{}'! {}", file_name, e);
            }
        };
    }

    match &cli.command {
        Commands::Ls => match File::open(file_name) {
            Ok(file) => {
                // println!("Opened file: {}", file_name);
                let reader = io::BufReader::new(file);
                let mut ln_number = 1;
                for line in reader.lines() {
                    match line {
                        Ok(line) => println!("{}: {}", ln_number, line),
                        Err(e) => eprintln!("Failed to read line: {}", e),
                    }
                    ln_number += 1;
                }
            }
            Err(e) => eprintln!("Failed to open file: {}", e),
        },
        Commands::Create { content } => {
            let mut file = match OpenOptions::new().append(true).open(file_name) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open file: {}", e);
                    return;
                }
            };

            if let Err(e) = writeln!(file, "{}", content) {
                eprintln!("Failed to write to todo list: {}", e);
            } else {
                println!("Added todo entry: {}", content);
            }
        }
        Commands::Rm => {
            match File::open(file_name) {
                Ok(file) => {
                    println!("Remove an entry:");
                    let mut ln_number = 1;
                    let reader = io::BufReader::new(file);
                    let mut lines: Vec<String> = Vec::new(); // Store lines in a vector
                    for line in reader.lines() {
                        match line {
                            Ok(line) => {
                                println!("{}: {}", ln_number, line);
                                lines.push(line); // Store each line in the vector
                            }
                            Err(e) => eprintln!("Failed to read line: {}", e),
                        }
                        ln_number += 1;
                    }

                    // Wait for user input
                    print!("Enter the entry number to delete: ");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    // Parse user input as a line number
                    let line_number_to_delete: usize = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Invalid input. Please enter a valid line number.");
                            return;
                        }
                    };

                    // Check if the line number is valid
                    if line_number_to_delete > 0 && line_number_to_delete <= lines.len() {
                        // Open the file again in write mode to truncate its content
                        let mut file = match File::create(file_name) {
                            Ok(file) => file,
                            Err(e) => {
                                eprintln!("Failed to open file: {}", e);
                                return;
                            }
                        };

                        // Write all lines except the one to be deleted back to the file
                        for (index, line) in lines.iter().enumerate() {
                            if index != line_number_to_delete - 1 {
                                writeln!(file, "{}", line).unwrap_or_else(|e| {
                                    eprintln!("Failed to write to file: {}", e);
                                    std::process::exit(1);
                                });
                            }
                        }
                        println!("Line {} deleted successfully.", line_number_to_delete);
                    } else {
                        eprintln!(
                            "Invalid line number. Please enter a number between 1 and {}.",
                            lines.len()
                        );
                    }
                }
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
        }
    }
}

// TODO
// [] Allow for arguments on rm command ie. 'todo rm 1'
// [] Research a better way to carry out rm command. Instead of rewriting the whole database.
