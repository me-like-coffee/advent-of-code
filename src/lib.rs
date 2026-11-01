pub mod utils;

pub use crate::utils::SolveFn;
pub use crate::utils::run_day;
pub use crate::utils::{Answer, Day, Year};

register_years!(
    year_2015, year_2018, year_2019, year_2020, year_2021, year_2022, year_2023, year_2024,
    year_2025
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_years() {
        assert_eq!(YEARS.first(), Some(&2015));
        assert!(YEARS.contains(&2025));
        assert!(YEARS.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn test_get_year_solver() {
        assert!(get_year_solver(2015, 1).is_some());
        assert!(get_year_solver(2019, 25).is_some());
        assert!(get_year_solver(2015, 2).is_none());
        assert!(get_year_solver(2014, 1).is_none());
    }
}
