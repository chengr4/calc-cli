pub fn calc_factorial(num: u64) -> u64 {
    let mut result = 1;

    for i in 1..=num {
        result *= i;
    }

    result
}

pub fn calc_combination(n: u64, k: u64) -> u64 {
    calc_factorial(n) / (calc_factorial(k) * calc_factorial(n - k))
}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn calc_factorial_of_3() {
      let fac = calc_factorial(3);

      assert_eq!(fac, 6);
    }

    #[test]
    fn c_five_chose_two() {
      let comb = calc_combination(5, 2);

      assert_eq!(comb, 10);
    }
}
