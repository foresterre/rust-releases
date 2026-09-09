use rust_releases_core::rust_release::date::Date;

pub fn parse(input: &str) -> Option<Date> {
    let bytes = input.as_bytes();

    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }

    let year = input[0..4].parse::<u16>().ok()?;
    let month = input[5..7].parse::<u8>().ok()?;
    let day = input[8..10].parse::<u8>().ok()?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }

    Some(Date::new(year, month, day))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_date() {
        assert_eq!(parse("2016-03-08"), Some(Date::new(2016, 3, 8)));
    }

    #[yare::parameterized(
        empty = { "" },
        too_short = { "2016-3-8" },
        too_long = { "02016-03-08" },
        separators = { "2016/03/08" },
        month_zero = { "2016-00-08" },
        month_thirteen = { "2016-13-08" },
        day_zero = { "2016-03-00" },
        day_thirty_two = { "2016-03-32" },
        not_a_number = { "20xy-03-08" },
    )]
    fn reject_an_invalid_date(input: &str) {
        assert_eq!(parse(input), None);
    }
}
