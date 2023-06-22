use clap::{Parser, Subcommand};
use rfcalc::{bitwise, math};

#[derive(Parser)]
struct CliArgs {
    #[command(subcommand)]
    /// The function to run
    calc_function: Functions,
}

#[derive(Subcommand)]
enum Functions {
    /// `factorial <NUM>`: Calculate the factorial of a number (max: 20)
    Factorial { num: u64 },
    /// `hw <NUM>`: Calculate the Hamming weight of a binary number
    HW { num: u64 },
    /// `c <N> <K>`: Calculate the combination of N choose K
    C { n: u64, k: u64 },
}

fn main() {
    let args = CliArgs::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &args.calc_function {
        Functions::Factorial { num } => {
            let factorial = math::calc_factorial(*num);
            println!("{}", factorial);
        }
        Functions::HW { num } => {
            let hw = bitwise::count_bit_hamming_weight(*num);
            println!("{}", hw);
        }
        Functions::C { n, k } => {
            let combination = math::calc_combination(*n, *k);
            println!("{}", combination);
        }
    }
}
