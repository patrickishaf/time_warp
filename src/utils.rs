pub fn get_days_in_month(month_number: u32, year: u32) -> u32 {
    let days_in_feb = get_days_in_february(year);
    /*
    The order of this array is very important. it should not be re-arranged.Every number here
    corresponds to the number of days of the month at month_number - 1.
    This means that January is month one, it has 31 days, and to get the number of days in January,
    you pass 1 into this function as month_number and it will return the number at the index 1 - 1
    which is 0. The number will be 31
     */
    let days_in_months = vec![31, days_in_feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    return days_in_months[(month_number as usize) - 1];
}

fn get_days_in_february(year: u32) -> u32 {
    if is_leap_year(year) {
        29
    } else {
        28
    }
}

fn is_leap_year(year: u32) -> bool {
    if year % 4 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::is_leap_year;

    #[test]
    fn is_leap_year_works() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2025));
    }
}