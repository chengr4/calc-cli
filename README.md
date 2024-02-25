# CLI calculation tools

> `rfcalc` stands for "R4 Calculations". R4 is my self representaion =_=

## Installation

1. Install Rust and Cargo will be installed, too
2. Run `cargo install rfcalc`

## Usage

- `rfcalc factorial <NUM>`: Calculate the factorial of a number (max: 20)
- `rfcalc hw <NUM>`: Calculate the Hamming weight of a binary number
- `rfcalc c <N> <K>`: Calculate the combination of `N` choose `K`

### `rfcalc bytes <expression to calculate>`

Quickly devide bytes

> only support units of lower case now

E.g.

```bash
rfcalc bytes "4mb/4kb"
1024 kb
```

## Testing

```bash
cargo run -- <subcommand> <args>
```
