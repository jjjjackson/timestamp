use std::ops::Index;

#[derive(Debug, PartialEq, Eq)]
pub struct TimeStamp {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
}

const BASE_YEAR: i64 = 2000; // 1970/1/1 ~ 2000/1/1
const DAYS_FROM_UNIX_TIME_TO_BASE_YEAR: i64 = 10957; // 1970/1/1 ~ 1999/12/31
const DAYS_PER_400Y: i64 = 146097; // 365 * 400 + 97
const DAYS_PER_100Y: i64 = 36524; // 365 * 100 + 24
const DAYS_PER_4Y: i64 = 1461; // 365 * 4 + 1
const DAYS_PER_Y: i64 = 365;
const DAYS_OF_MONTH_FROM_JAN: [i64; 13] =
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
const SECONDS_PER_DAY: i64 = 86400; // 60 * 60 * 24
const SECONDS_PER_HOUR: i64 = 3600; // 60 * 60
const SECONDS_PER_MINUTE: i64 = 60;

impl TimeStamp {
    // fn now() -> TimeStamp {
    //     let sys_time = SystemTime::now();
    //     return TimeStamp::to_timestamp(sys_time);
    // }

    fn to_timestamp(total_seconds_from_unix_time: i64) -> TimeStamp {
        // let total_seconds_from_unix_time = sys_time
        //     .duration_since(SystemTime::UNIX_EPOCH)
        //     .map_or(
        //         Duration::ZERO,
        //     |f|f
        //     ).as_secs() as i64;

        let mut total_days_from_unix_time = total_seconds_from_unix_time / SECONDS_PER_DAY;
        let mut remained_seconds = total_seconds_from_unix_time % SECONDS_PER_DAY;
        if remained_seconds < 0 {
            total_days_from_unix_time -= 1;
            remained_seconds += SECONDS_PER_DAY;
        }

        let mut days = total_days_from_unix_time - DAYS_FROM_UNIX_TIME_TO_BASE_YEAR;

        let mut quad_century_cycles: i64 = days / DAYS_PER_400Y;
        days = days % DAYS_PER_400Y;
        if days < 0 {
            days += DAYS_PER_400Y;
            quad_century_cycles -= 1;
        }

        let mut century_cycles = days / DAYS_PER_100Y;
        if century_cycles == 4 {
            century_cycles -= 1;
        }
        days -= century_cycles * DAYS_PER_100Y;

        let mut quad_cycles = days / DAYS_PER_4Y;
        if quad_cycles == 25 {
            quad_cycles -= 1;
        }
        days -= quad_cycles * DAYS_PER_4Y;

        let mut remained_years = days / DAYS_PER_Y;
        if remained_years == 4 {
            remained_years -= 1;
        }
        days -= remained_years * DAYS_PER_Y;

        let is_this_year_a_leap_year = remained_years == 0
            && (quad_cycles != 0 || century_cycles == 0 || quad_century_cycles != 0);

        let mut remained_days = days + is_this_year_a_leap_year as i64;

        if remained_days >= DAYS_PER_Y + is_this_year_a_leap_year as i64 {
            remained_days -= DAYS_PER_Y + is_this_year_a_leap_year as i64;
        }

        let current_year = BASE_YEAR
            + remained_years
            + 4 * quad_cycles
            + 100 * century_cycles
            + 400 * quad_century_cycles;

        let mut current_month: i64 = 0;
        for i in 1..13 {
            if TimeStamp::get_days_from_jan(i, is_this_year_a_leap_year) >= remained_days {
                current_month = i;
                break;
            }
        }

        let previous_month = current_month - 1;
        let current_day =
            remained_days - TimeStamp::get_days_from_jan(previous_month, is_this_year_a_leap_year);

        let current_hour = remained_seconds / SECONDS_PER_HOUR;
        let remained_seconds = remained_seconds % SECONDS_PER_HOUR;
        let current_minute = remained_seconds / SECONDS_PER_MINUTE;
        let current_second = remained_seconds % SECONDS_PER_MINUTE;

        return TimeStamp {
            year: current_year,
            month: current_month,
            day: current_day,
            hour: current_hour,
            minute: current_minute,
            second: current_second,
        };
    }

    fn get_days_from_jan(month: i64, is_leap_year: bool) -> i64 {
        if month >= 2 && is_leap_year {
            *DAYS_OF_MONTH_FROM_JAN.index(month as usize) + 1
        } else {
            *DAYS_OF_MONTH_FROM_JAN.index(month as usize)
        }
    }
}

#[cfg(test)]
extern crate test_case;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(-11670912000,  TimeStamp{ year: 1600, month: 3, day: 1, hour: 0, minute: 0, second: 0}  ; "1600/03/01 0:0:0")]
    #[test_case(-2203891200,  TimeStamp{ year: 1900, month: 3, day: 1, hour: 0, minute: 0, second: 0}  ; "1900/03/01 0:0:0")]
    #[test_case(-2077660800,  TimeStamp{ year: 1904, month: 3, day: 1, hour: 0, minute: 0, second: 0}  ; "1904/03/01 0:0:0")]
    #[test_case(983318400,  TimeStamp{ year: 2001, month: 2, day: 28, hour: 0, minute: 0, second: 0}  ; "2001/02/28 0:0:0")]
    #[test_case(983404800,  TimeStamp{ year: 2001, month: 3, day: 1, hour: 0, minute: 0, second: 0}  ; "2001/03/01 0:0:0")]
    #[test_case(1078099200,  TimeStamp{ year: 2004, month: 3, day: 1, hour: 0, minute: 0, second: 0}  ; "2004/03/01 0:0:0")]
    #[test_case(4107542400,  TimeStamp{ year: 2100, month: 3, day: 1, hour: 0, minute: 0, second: 0}  ; "2100/03/01 0:0:0")]
    #[test_case(13569465600,  TimeStamp{ year: 2400, month: 1, day: 1, hour: 0, minute: 0, second: 0}  ; "2400/01/01 0:0:0")]
    #[test_case(13572057600,  TimeStamp{ year: 2400, month: 1, day: 31, hour: 0, minute: 0, second: 0}  ; "2400/01/31 0:0:0")]
    #[test_case(13574563200,  TimeStamp{ year: 2400, month: 2, day: 29, hour: 0, minute: 0, second: 0}  ; "2400/02/29 0:0:0")]
    #[test_case(13574649600,  TimeStamp{ year: 2400, month: 3, day: 1, hour: 0, minute: 0, second: 0}  ; "2400/03/01 0:0:0")]
    fn test_to_timestamp(total_seconds_from_unix_time: i64, expected: TimeStamp) {
        let got = TimeStamp::to_timestamp(total_seconds_from_unix_time);

        assert_eq!(expected, got);
    }
}
