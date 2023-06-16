pub fn count_bit_hamming_weight(num: u64) -> u32 {
    let mut count = 0;
    let mask = 1;
    let mut num_copy = num;

    for _ in 0..64 {
        if (mask & num_copy) != 0 {
            count += 1;
        }
        num_copy >>= 1;
    }

    count
}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn test_count_bit_hamming_weight() {
      let hw = count_bit_hamming_weight(3);

      assert_eq!(hw, 2);
    }
}
