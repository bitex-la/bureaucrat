/* Simple multiply digit-by-digit checksums. values is all digits already */
pub fn checksum(values: &str, mults: Vec<u32>) -> u32 {
    values.chars()
        .map(|c| c.to_digit(10).unwrap())
        .zip(mults.iter())
        .map(|(a,b)| a * b)
        .sum()
}

pub fn is_all_numeric(value: &str, length: usize) -> bool {
    value.chars().count() == length && value.chars().all(|c| c.is_numeric())
}
