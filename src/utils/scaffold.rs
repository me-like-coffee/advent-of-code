//! Source-editing helpers for the `generate` command.
//!
//! They register new modules in the `register_days!` and `register_years!` calls.

use anyhow::{Result, anyhow};

/// Maximum line width, matching rustfmt's default.
const MAX_WIDTH: usize = 100;

/// Maximum width of a call's arguments on one line, matching rustfmt's `fn_call_width`.
const FN_CALL_WIDTH: usize = 60;

/// Adds `item` to the top-level `macro_name!(…);` call in `source`.
///
/// The items are kept sorted. Returns `None` if `item` is already registered.
///
/// # Errors
/// Returns an error if `source` has no such call or the call holds anything but identifiers.
pub fn register_in_macro(source: &str, macro_name: &str, item: &str) -> Result<Option<String>> {
    let open = format!("{macro_name}!(");
    let start = source
        .find(&open)
        .ok_or_else(|| anyhow!("No `{macro_name}!(` call found"))?;
    let args_start = start + open.len();
    let args_len = source[args_start..]
        .find(')')
        .ok_or_else(|| anyhow!("Unclosed `{macro_name}!(` call"))?;
    let args_end = args_start + args_len;

    let mut items: Vec<&str> = source[args_start..args_end]
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if let Some(bad) = items.iter().find(|s| !is_ident(s)) {
        return Err(anyhow!("Unexpected `{bad}` in `{macro_name}!` call"));
    }
    if items.contains(&item) {
        return Ok(None);
    }
    items.push(item);
    items.sort_unstable();

    let mut result = String::with_capacity(source.len() + item.len() + 8);
    result.push_str(&source[..start]);
    result.push_str(&format_macro_call(macro_name, &items));
    result.push_str(&source[args_end + 1..]);
    Ok(Some(result))
}

/// Formats `macro_name!(items)` the way rustfmt lays out a call followed by `;`.
///
/// The call stays on one line if its arguments fit in [`FN_CALL_WIDTH`] and the
/// line fits in [`MAX_WIDTH`]. Otherwise the items are packed onto indented lines
/// between the parentheses, which is how rustfmt lays out short items (at most
/// 10 characters, as day and year module names are).
#[must_use]
pub fn format_macro_call(macro_name: &str, items: &[&str]) -> String {
    let args = items.join(", ");
    let single = format!("{macro_name}!({args})");
    if args.len() <= FN_CALL_WIDTH && single.len() < MAX_WIDTH {
        return single;
    }

    let mut out = format!("{macro_name}!(\n");
    let mut line = String::new();
    for (i, item) in items.iter().enumerate() {
        let sep = if i + 1 < items.len() { "," } else { "" };
        if !line.is_empty() && line.len() + 1 + item.len() + sep.len() > MAX_WIDTH {
            out.push_str(&line);
            out.push('\n');
            line.clear();
        }
        if line.is_empty() {
            line.push_str("    ");
        } else {
            line.push(' ');
        }
        line.push_str(item);
        line.push_str(sep);
    }
    out.push_str(&line);
    out.push_str("\n)");
    out
}

fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_in_macro_inserts_sorted() -> Result<()> {
        let source = "use crate::register_days;\n\nregister_days!(day_01, day_05);\n";
        let result = register_in_macro(source, "register_days", "day_03")?;
        assert_eq!(
            result.as_deref(),
            Some("use crate::register_days;\n\nregister_days!(day_01, day_03, day_05);\n")
        );
        Ok(())
    }

    #[test]
    fn test_register_in_macro_already_present() -> Result<()> {
        let source = "register_days!(day_01, day_05);\n";
        assert_eq!(register_in_macro(source, "register_days", "day_05")?, None);
        Ok(())
    }

    #[test]
    fn test_register_in_macro_multiline() -> Result<()> {
        let source = "pub mod intcode;\n\nregister_days!(\n    day_02, day_05,\n    day_07\n);\n";
        let result = register_in_macro(source, "register_days", "day_06")?;
        assert_eq!(
            result.as_deref(),
            Some("pub mod intcode;\n\nregister_days!(day_02, day_05, day_06, day_07);\n")
        );
        Ok(())
    }

    #[test]
    fn test_register_in_macro_empty_call() -> Result<()> {
        let result = register_in_macro("register_years!();\n", "register_years", "year_2016")?;
        assert_eq!(result.as_deref(), Some("register_years!(year_2016);\n"));
        Ok(())
    }

    #[test]
    fn test_register_in_macro_missing_call() {
        assert!(register_in_macro("fn main() {}\n", "register_days", "day_01").is_err());
    }

    #[test]
    fn test_register_in_macro_rejects_non_identifiers() {
        let source = "register_days!(day_01, \"x\");\n";
        assert!(register_in_macro(source, "register_days", "day_02").is_err());
    }

    #[test]
    fn test_format_macro_call_single_line() {
        assert_eq!(
            format_macro_call("register_days", &["day_01", "day_02"]),
            "register_days!(day_01, day_02)"
        );
    }

    #[test]
    fn test_format_macro_call_fn_call_width() {
        // Seven day names are 54 characters of arguments, eight are 62.
        let names: Vec<String> = (1..=8).map(|d| format!("day_{d:02}")).collect();
        let items: Vec<&str> = names.iter().map(String::as_str).collect();
        assert_eq!(
            format_macro_call("register_days", &items[..7])
                .lines()
                .count(),
            1
        );
        assert_eq!(
            format_macro_call("register_days", &items),
            "register_days!(\n    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08\n)"
        );
    }

    #[test]
    fn test_format_macro_call_wraps_at_max_width() {
        let names: Vec<String> = (1..=25).map(|d| format!("day_{d:02}")).collect();
        let items: Vec<&str> = names.iter().map(String::as_str).collect();
        let formatted = format_macro_call("register_days", &items);

        let lines: Vec<&str> = formatted.lines().collect();
        assert_eq!(lines.first(), Some(&"register_days!("));
        assert_eq!(lines.last(), Some(&")"));
        assert!(lines.iter().all(|l| l.len() <= MAX_WIDTH));
        assert_eq!(
            lines[1],
            "    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12,"
        );
        assert!(lines[lines.len() - 2].ends_with("day_25"));
    }
}
