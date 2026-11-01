use crate::utils::{Answer, Day, Year, read_input};
use anyhow::Result;

/// The list holds 0..=255 for the puzzle input.
const INPUT_LEN: u8 = 255;

fn reverse_elements(elements: &mut [u8], start: usize, length: u8) {
    let mut l = start;
    let mut r = (l + length as usize - 1) % elements.len();
    for _ in 0..length / 2 {
        elements.swap(l, r);
        r = if r == 0 { elements.len() - 1 } else { r - 1 };
        l = (l + 1) % elements.len();
    }
}

fn knot_hash(elements: &[u8], lengths: &[u8], curr: usize, skip: usize) -> (Vec<u8>, usize, usize) {
    let mut elements = elements.to_vec();
    let mut curr = curr;
    let mut skip = skip;

    for &length in lengths {
        if length > 1 {
            reverse_elements(&mut elements, curr, length);
        }
        curr = (curr + length as usize + skip) % elements.len();
        skip += 1;
    }

    (elements, curr, skip)
}

fn part_one(data: &str, last_element: u8) -> Result<u16> {
    let lengths = data
        .split(',')
        .map(|t| t.parse())
        .collect::<Result<Vec<u8>, _>>()?;
    let elements: Vec<_> = (0..=last_element).collect();

    let (elements, _, _) = knot_hash(&elements, &lengths, 0, 0);

    Ok(elements[0] as u16 * elements[1] as u16)
}

fn dense_hash(sparse_hash: &[u8]) -> Vec<u8> {
    let mut dense = Vec::new();
    let mut i = 0;
    while i < sparse_hash.len() {
        let mut value = 0;
        for _ in 0..16 {
            value ^= sparse_hash[i];
            i += 1;
        }
        dense.push(value);
    }

    dense
}

fn nibble_to_digit(nibble: u8) -> char {
    match nibble {
        0..=9 => (nibble + b'0') as char,
        10..=15 => (nibble - 10 + b'a') as char,
        _ => unreachable!(),
    }
}

fn to_hexadecimal(dense_hash: &[u8]) -> String {
    let mut hexadecimal = String::new();

    for &num in dense_hash {
        hexadecimal.push(nibble_to_digit(num >> 4));
        hexadecimal.push(nibble_to_digit(num & 0x0f));
    }

    hexadecimal
}

fn part_two(data: &str, last_element: u8) -> String {
    let mut lengths: Vec<_> = data.bytes().collect();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut elements: Vec<_> = (0..=last_element).collect();
    let mut skip = 0;
    let mut curr = 0;

    for _ in 0..64 {
        (elements, curr, skip) = knot_hash(&elements, &lengths, curr, skip);
    }

    to_hexadecimal(&dense_hash(&elements))
}

/// Solves Year 2017, Day 10: Knot Hash.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(10))?;
    Ok((
        u64::from(part_one(&data, INPUT_LEN)?).into(),
        part_two(&data, INPUT_LEN).into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    /// The example's list holds 0..=4.
    const TEST1_LEN: u8 = 4;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        let data = read_example(Year(2017), Day(10), 1)?;
        assert_eq!(part_one(&data, TEST1_LEN)?, 12);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 3770.into());
        assert_eq!(p2, "a9d0e68649d0174c8756a59ba21d4dc6".into());
        Ok(())
    }
}
