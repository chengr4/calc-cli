use regex::Regex;

pub fn calc_factorial(num: u64) -> u64 {
    let mut result = 1;

    for i in 1..=num {
        result *= i;
    }

    result
}

pub fn calc_combination(n: u64, k: u64) -> u64 {
    let mut numerator = 1;
    let mut denominator = 1;

    // loop from n to k
    for i in (n - k + 1)..=n {
        numerator *= i;
    }

    // loop from 1 to k
    for i in 1..=k {
        denominator *= i;
    }

    numerator / denominator
}

/// B: byte
#[derive(PartialEq, PartialOrd)]
enum ByteUnit {
    B,
    KB,
    MB,
    GB,
    TB,
}

impl ByteUnit {
    fn to_bits(&self) -> u64 {
        match self {
            ByteUnit::B => 1,
            ByteUnit::KB => 2u64.pow(10), // 2^10
            ByteUnit::MB => 2u64.pow(20), // 2^20
            ByteUnit::GB => 2u64.pow(30), // 2^30
            ByteUnit::TB => 2u64.pow(40), // 2^40
        }
    }
}

pub fn devide_bytes(expression: &str) -> String {
    let re = Regex::new(r"(?<num1>\d+)\s*(?<unit1>byte|bytes|kb|mb|gb|tb)\s*(?<op>/)\s*(?<num2>\d+)\s*(?<unit2>byte|bytes|kb|mb|gb|tb)").unwrap();
    if let Some(caps) = re.captures(expression) {
        let num1: u64 = caps.name("num1").unwrap().as_str().parse().unwrap();
        let unit1 = match caps.name("unit1").unwrap().as_str() {
            "byte" => ByteUnit::B,
            "bytes" => ByteUnit::B,
            "kb" => ByteUnit::KB,
            "mb" => ByteUnit::MB,
            "gb" => ByteUnit::GB,
            "tb" => ByteUnit::TB,
            _ => return "Wrong input".to_string(),
        };
        let num2: u64 = caps.name("num2").unwrap().as_str().parse().unwrap();
        let unit2 = match caps.name("unit2").unwrap().as_str() {
            "byte" => ByteUnit::B,
            "bytes" => ByteUnit::B,
            "kb" => ByteUnit::KB,
            "mb" => ByteUnit::MB,
            "gb" => ByteUnit::GB,
            "tb" => ByteUnit::TB,
            _ => return "Wrong input".to_string(),
        };

        if unit1 < unit2 {
            return "Wrong input".to_string();
        }

        let result = match caps.name("op").unwrap().as_str() {
            "/" => num1 * unit1.to_bits() / (num2 * unit2.to_bits()),
            _ => return "Wrong input".to_string(),
        };

        return format!("{} times", result);
    }
    "Wrong input".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_factorial_of_five() {
        let fac = calc_factorial(3);

        assert_eq!(fac, 6);
    }

    #[test]
    fn c_five_chose_two() {
        let comb = calc_combination(5, 2);

        assert_eq!(comb, 10);
    }

    #[test]
    fn c_thiry_chose_two() {
        let comb = calc_combination(30, 2);

        assert_eq!(comb, 435);
    }

    #[test]
    fn four_mb_devide_by_four_kb() {
        let bytes = devide_bytes("4mb / 4kb");

        assert_eq!(bytes, "1024 times");
    }
}
