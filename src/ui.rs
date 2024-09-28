pub mod cli {
    use std::io::{self};
    use chrono::{DateTime, FixedOffset, ParseError};

    use crate::{error::ui::parse_error, helper::{calendar::get_timestamp_localized, output::splitted_by_sub_len}};

    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }

    pub fn get_input_desired_date(text: String) -> Result<DateTime<FixedOffset>, ParseError> {

        let date_start: String = get_input(&text);
        let date_time_start  = date_start.replace("-", "") + "T000000Z";
        let desired_date_start = match get_timestamp_localized(date_time_start.to_owned()) {
            Ok(date) => date,
            Err(error) => {
                parse_error(error);
                return Err(error);
            }
        };
        return Ok(desired_date_start);
    }

    pub fn get_header(cal_url: String) {
        println!("__________________________________________________________");
        println!("                                                          ");
        println!("ics2cvs4all ----------- 2024.10.1 ----------- by M. Kittel");
        println!("                   verwendeter Kalender:                  ");
        splitted_by_sub_len(cal_url, 58);
        println!("----------------------------------------------------------");
        println!("Bitte nachfolgend die zu exportierende Zeitspanne eingeben");
        println!("__________________________________________________________");        
    }

}