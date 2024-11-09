pub mod calendar {
    use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, ParseError};

    fn is_dst(date: DateTime<FixedOffset>, year: i32) -> bool {
        let date_of_last_sunday_in_october = get_last_sunday_in_october(year);
        let date_of_last_sunday_in_march: NaiveDate = get_last_sunday_in_march(year);
    
        return date.naive_local().date().ge(&date_of_last_sunday_in_march) && !date.naive_local().date().ge(&date_of_last_sunday_in_october);
    }
    pub fn get_short_weekday(timestamp: DateTime<FixedOffset>) -> String {
        let weekday_as_number = timestamp.naive_local().weekday().num_days_from_monday();
        match weekday_as_number {
            0 => return "Mo".to_string(),
            1 => return "Di".to_string(),
            2 => return "Mi".to_string(),
            3 => return "Do".to_string(),
            4 => return "Fr".to_string(),
            5 => return "Sa".to_string(),
            6 => return "So".to_string(),
            _ => return "??".to_string(),
        }
    }    
    fn get_last_sunday_in_october(year: i32) -> NaiveDate {
        let date_of_last_day_in_october = NaiveDate::from_ymd_opt(year, 10, 31).unwrap();
        // calc last sunday
        let last_sunday = date_of_last_day_in_october
            - chrono::Duration::days((date_of_last_day_in_october.weekday().num_days_from_sunday()) as i64);
        last_sunday
    }
    
    fn get_last_sunday_in_march(year: i32) -> NaiveDate {
        let date_of_last_day_in_march = NaiveDate::from_ymd_opt(year, 3, 31).unwrap();
        // calc last sunday
        let last_sunday = date_of_last_day_in_march
            - chrono::Duration::days((date_of_last_day_in_march.weekday().num_days_from_sunday()) as i64);
        last_sunday
    }
    pub fn get_timestamp_localized(timestamp: String) -> Result<DateTime<FixedOffset>, ParseError> {

        let mut timezone: FixedOffset = FixedOffset::east_opt(1 * 3600).unwrap();
    
        //20241027T100000Z
        let mut fullfillment = "".to_string();
    
        // for whole day events fullfill the time
        if timestamp.len() == 8 {
            fullfillment = "T000000Z".to_owned();
        }
    
        let timestamp_utc = match NaiveDateTime::parse_from_str((timestamp + &fullfillment).as_str(), "%Y%m%dT%H%M%SZ") {
            Ok(timestamp_utc) => timestamp_utc.and_utc(),
            Err(error) => return Err(error),
        };
    
        if is_dst(timestamp_utc.with_timezone(&timezone), timestamp_utc.year()) {
            timezone = FixedOffset::east_opt(2 * 3600).unwrap();
        } 
        
        // it's an whole day event, no timezone change since it was already fullfilled
        if fullfillment.len() > 0 {
            timezone = FixedOffset::east_opt(0 * 3600).unwrap();
        }
    
        return Ok(timestamp_utc.with_timezone(&timezone));
    }     

}

pub mod output {
    use core::str;

    pub fn splitted_by_sub_len (text: String, sub_len: usize) {
        let sub_len = sub_len;
        let subs = text.as_bytes()
        .chunks(sub_len)
        .map(|buf| unsafe { str::from_utf8_unchecked(buf) })
        .collect::<Vec<&str>>();
        for sub in subs.iter(){ // it works even if .iter() is not written
            println!("{}", sub);
        }
    }

}
