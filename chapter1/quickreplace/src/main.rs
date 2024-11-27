use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {}",
                "Error:".red(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to replace '{}' with '{}' in '{}': {}",
                "Error:".red(),
                args.target,
                args.replacement,
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {
            println!(
                "{} successfully replaced '{}' with '{}' in '{}', and wrote to '{}'.",
                "Success:".green(),
                args.target,
                args.replacement,
                args.filename,
                args.output
            );
        }
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {}",
                "Error:".red(),
                args.output,
                e
            );
            std::process::exit(1);
        }
    }
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error:".red(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let re = Regex::new(target)?;
    Ok(re.replace_all(text, replacement).to_string())
}
