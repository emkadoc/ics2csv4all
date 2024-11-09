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
                confirm_with_enter();
                return Err(error);
            }
        };
        return Ok(desired_date_start);
    }

    pub fn get_header(cal_url: String) {
        println!("____________________________________________________________");
        println!("                                                            ");
        print!("emkadoc/ics2csv4all                                 v");
        println!(env!("CARGO_PKG_VERSION"));
        println!("____________________________________________________________"); 
        println!("                                                            ");
        println!("Online-Kalender:                                            ");
        splitted_by_sub_len(cal_url, 60);
        println!("____________________________________________________________"); 
        println!("                                                            ");       
        println!("-Bitte nachfolgend die zu exportierende Zeitspanne eingeben-");
        println!("                                                            ");
    }

    pub fn confirm_with_enter() {
        println!("Bitte Eingabetaste betätigen um Fenster zu schließen.");
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);
        let _ = result.is_ok();
    }

}