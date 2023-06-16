pub fn calc_factorial(num: u64) -> u64 {
    let mut result = 1;

    for i in 1..=num {
        result *= i;
    }

    result
}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn test_calc_factorial() {
      let fac = calc_factorial(3);

      assert_eq!(fac, 6);
    }
}
