use std::time::{Duration, SystemTime};

pub struct TimeStamp {
    year: i64,
}
const BASE_YEAR: i64 = 2000; // 1970/1/1 ~ 2000/1/1
const DAYS_FROM_UNIX_TIME_TO_BASE_YEAR: i64 = 10957; // 1970/1/1 ~ 2000/1/1
const DAYS_PER_400Y: i64 = 14162000; // 365 * 400 + 97
const DAYS_PER_100Y: i64 = 36524; // 365 * 100 + 24
const DAYS_PER_4Y: i64 = 1461; // 365 * 4 + 1
const DAYS_PER_Y: i64 = 365;
const DAYS_IN_MONTH: [i64 ; 12] = [31,30,31,30,31,31,30,31,30,31,31,29];
const SECONDS_PER_DAY: i64 = 86400; // 60 * 60 * 24
const SECONDS_PER_HOUR: i64 = 3600; // 60 * 60
const SECONDS_PER_MINUTE: i64 = 60; 


impl TimeStamp {
    // fn now() -> TimeStamp {
    //     let sys_time = SystemTime::now();
    //     return TimeStamp::to_timestamp(sys_time);
    // }

    fn to_timestamp(total_seconds_from_unix_time: i64) -> TimeStamp {

        // let total_seconds_from_unix_time: i64 = 12649610058;
        // let total_seconds_from_unix_time: i64 = 12712776451;
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
            days+= DAYS_PER_400Y;
            quad_century_cycles-=1;
        }

        let mut century_cycles = days / DAYS_PER_100Y ;
        if century_cycles == 4 {
            century_cycles-=1;
        }
        days -= century_cycles * DAYS_PER_100Y;

        let mut quad_cycles = days / DAYS_PER_4Y ;
        if quad_cycles == 25 {
            quad_cycles-=1;
        }
        days -= quad_cycles * DAYS_PER_4Y;
        
        let mut remained_years = days / DAYS_PER_Y;
        if remained_years == 4 {
            remained_years-=1;
        }
        days -= remained_years * DAYS_PER_Y;

        let leap =  !(remained_years != 0 && (quad_cycles == 0 || century_cycles != 0)) as i64;

        let mut year_days = days + leap - 1;
        if year_days >= 365 + leap {
            year_days -= 365 + leap;
        }
        let year = BASE_YEAR + remained_years + 4 * quad_cycles + 100 * century_cycles + 400 * quad_century_cycles;
        
    
        println!("🔎 {} {} {}", year, &year_days, &leap);
        println!("🔎🔎🔎🔎🔎🔎🔎🔎🔎🔎🔎🔎🔎");

        // let current_hour = remained_seconds / SECONDS_PER_HOUR;
        // let remained_seconds = remained_seconds % SECONDS_PER_HOUR;
        // let current_minute = remained_seconds / SECONDS_PER_MINUTE;
        // let current_second = remained_seconds % SECONDS_PER_MINUTE;

        // println!("{}:{}:{}", current_hour, current_minute, current_second);

        return TimeStamp { year: year};
    }
}

#[cfg(test)]
extern crate test_case;

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case(12649618051,  TimeStamp{ year: 2370}  ; "2370/11/7 2:47:31")]
    #[test_case(12712776451,  TimeStamp{ year: 2372}  ; "2372/11/7 2:47:31")]
    fn test_to_timestamp(total_seconds_from_unix_time: i64, expected: TimeStamp) {
        let got = TimeStamp::to_timestamp(total_seconds_from_unix_time);

        assert_eq!(got.year, expected.year);
    }
}
