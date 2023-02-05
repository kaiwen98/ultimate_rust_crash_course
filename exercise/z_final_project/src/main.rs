// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");
use std::fmt::{self, Debug, Display};
mod parser;
mod image;

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        print_usage_and_exit();
    }

    let parser = parser::Parser{};

    let result = parser.parse(&mut args);

    match result {
        Ok((command_op, command_line_args)) => {
            match (command_op.command)(command_line_args) {
                Ok(_) => (),
                Err(err_msg) => {
                    println!("{}", err_msg);
                    println!("[ERR] Operation <{}> Fail!", command_op.command_type);

                    return;
                }
            };
            println!("[SUC] Operation <{}> Success!", command_op.command_type);
        },
        Err(e) => { 
            println!("[ERR] Parsing Error! {}", e);
            return;
        }
    }

}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

