//! Parsing puzzle inputs that are drawn as grids, one row per line.

/// Splits the input into rows of characters.
///
/// Rows keep their own lengths, so a ragged input stays ragged.
#[must_use]
pub fn char_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

/// Splits the input into rows of bytes.
///
/// Rows keep their own lengths, so a ragged input stays ragged.
#[must_use]
pub fn byte_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_grid() {
        assert_eq!(char_grid("#.\n.#\n"), vec![vec!['#', '.'], vec!['.', '#']]);
    }

    #[test]
    fn test_byte_grid() {
        assert_eq!(byte_grid("ab\ncd"), vec![b"ab".to_vec(), b"cd".to_vec()]);
    }

    #[test]
    fn test_ragged_rows_keep_their_lengths() {
        let grid = char_grid("abc\nd\n\nef");
        let lengths: Vec<usize> = grid.iter().map(Vec::len).collect();
        assert_eq!(lengths, vec![3, 1, 0, 2]);
    }

    #[test]
    fn test_windows_line_endings() {
        assert_eq!(
            byte_grid("ab\r\ncd\r\n"),
            vec![b"ab".to_vec(), b"cd".to_vec()]
        );
    }

    #[test]
    fn test_empty_input() {
        assert!(char_grid("").is_empty());
        assert!(byte_grid("").is_empty());
    }
}
