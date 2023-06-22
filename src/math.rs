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
  for i in (n-k+1)..=n {
    numerator *= i;
  }

  // loop from 1 to k
  for i in 1..=k {
    denominator *= i;
  }

  numerator / denominator
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
