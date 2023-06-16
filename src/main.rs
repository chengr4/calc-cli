use clap::Parser;
use rfcalc::{math, bitwise};

#[derive(Parser)]
struct CliArgs {
    /// The function to run
    function: String,
}

fn main() {
    let args = CliArgs::parse();

    match args.function.as_str() {
        "bitwise" => {
            println!("Running bitwise function");
            println!("-------------------------");
            println!("count_bit_hamming_weight(3) = {}", bitwise::count_bit_hamming_weight(3));
        },
        "math" => {
            println!("Running math function");
            println!("-------------------------");
            println!("calc_factorial(3) = {}", math::calc_factorial(3));
        },
        _ => {
            println!("Unknown function: {}", args.function);
        }
    }
}
