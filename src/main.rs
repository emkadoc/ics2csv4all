pub mod ui;
pub mod file;
pub mod error;
pub mod helper;

use helper::calendar::{get_short_weekday, get_timestamp_localized};
use chrono::{DateTime, FixedOffset, Local};
use error::csv::write_error;
use ui::cli::{confirm_with_enter, get_header, get_input_desired_date};
use file::json::{delete, read_config};


fn main() {

    let file_name = "basic";

    let config = read_config().unwrap();
    
    get_header(config.cal_url.clone());

    let desired_date_start = match get_input_desired_date("1. Startdatum (Schema: JJJJ-MM-TT):".to_string()) {
        Ok(date) => date,
        Err(_) => {
            return;
        }
    };

    let desired_date_end = match get_input_desired_date("2. Endedatum (Schema: JJJJ-MM-TT):".to_string()) {
        Ok(date) => date,
        Err(_) => {
            return;
        }
    };

    println!("Downloading...");
    
    // Local File
    //let file = std::fs::read_to_string(file_name.to_owned() + ".ics").unwrap();
    // Remote File
    let file = reqwest::blocking::get(config.cal_url).unwrap().text().unwrap();
    
    let ics_bytes = file.as_bytes();
    let ics_reader = ical::PropertyParser::from_reader(ics_bytes);

    let mut found = false;

    let writer_result = csv::WriterBuilder::new().delimiter(b'\t').from_path(file_name.to_owned() + ".csv");
    
    let mut csv_writer = match writer_result {
        Ok(writer) => writer,
        Err(_) => todo!(),
    };

    let mut location = "".to_string().to_owned();
    let mut status = "".to_string().to_owned();
    let mut timestamp_start: DateTime<FixedOffset> = Local::now().fixed_offset();
    let mut timestamp_end: DateTime<FixedOffset> = Local::now().fixed_offset();
    let datetime_format_long = "%d.%m.%Y %H:%M";
    let datetime_format_short = "%Y%m%d%H";

     // write summary
     match csv_writer.write_field("SUMMARY") {
        Ok(record_result) => record_result,
        Err(error) => { write_error(error); return; }
    };   
    // write short timestamp
    match csv_writer.write_field("DTSTART (sortable)") {
        Ok(record_result) => record_result,
        Err(error) => { write_error(error); return; }
    };              
    // write start timestamp
    match csv_writer.write_field("DTSTART") {
        Ok(record_result) => record_result,
        Err(error) => { write_error(error); return; }
    };  
    // write end timestamp
    match csv_writer.write_field("DTEND") {
        Ok(record_result) => record_result,
        Err(error) => { write_error(error); return; }
    };

    // write location
    match csv_writer.write_field("LOCATION") {
        Ok(record_result) => record_result,
        Err(error) => { write_error(error); return; }
    };       

    // write location
    match csv_writer.write_field("STATUS") {
        Ok(record_result) => record_result,
        Err(error) => { write_error(error); return; }
    };                   

    // write carriage return
    match csv_writer.write_record(&[""]) {
        Ok(record_result) => record_result,
        Err(error) => { write_error(error); return; }
    };


    for result_ics in ics_reader {

        let property_ics = match result_ics {
            Ok(property_ics) => property_ics,
            Err(error) => panic!("Throw error: {}", error),
        };
        
        let property_ics_value = (property_ics.value).unwrap();

        if property_ics.name == "DTSTART" && (property_ics_value.contains("Z") || property_ics_value.len() == 8) {
            let result_date = get_timestamp_localized(property_ics_value.to_string()).unwrap(); 
            if result_date.ge(&desired_date_start) && !result_date.gt(&desired_date_end) {
                timestamp_start = get_timestamp_localized(property_ics_value.to_string()).unwrap();
            }
        };

        if property_ics.name == "DTEND" && (property_ics_value.contains("Z") || property_ics_value.len() == 8) {
            let result_date = get_timestamp_localized(property_ics_value.to_string()).unwrap(); 
            if result_date.ge(&desired_date_start) && !result_date.gt(&desired_date_end) {
                timestamp_end = get_timestamp_localized(property_ics_value.to_string()).unwrap();
                found = true;
            }
        }; 

        if property_ics.name == "LOCATION" && found == true {
            location = property_ics_value.clone().to_string();
        }        

        if property_ics.name == "STATUS" && found == true {
            status = property_ics_value.clone().to_string();
        }               

        if property_ics.name == "SUMMARY" && found == true {
            // write summary
            match csv_writer.write_field(property_ics_value.replace("\\", "")) {
                Ok(record_result) => record_result,
                Err(error) => { write_error(error); return; }
            };   
            // write short timestamp
            match csv_writer.write_field(timestamp_start.format(datetime_format_short).to_string()) {
                Ok(record_result) => record_result,
                Err(error) => { write_error(error); return; }
            };              
            // write start timestamp
            match csv_writer.write_field(get_short_weekday(timestamp_start) + " " + &timestamp_start.format(datetime_format_long).to_string()) {
                Ok(record_result) => record_result,
                Err(error) => { write_error(error); return; }
            };  
            // write end timestamp
            match csv_writer.write_field(get_short_weekday(timestamp_end) + " " + &timestamp_end.format(datetime_format_long).to_string()) {
                Ok(record_result) => record_result,
                Err(error) => { write_error(error); return; }
            };

            // write location
            match csv_writer.write_field(location.clone()) {
                Ok(record_result) => record_result,
                Err(error) => { write_error(error); return; }
            };       

            // write location
            match csv_writer.write_field(status.clone()) {
                Ok(record_result) => record_result,
                Err(error) => { write_error(error); return; }
            };                   

            // write carriage return
            match csv_writer.write_record(&[""]) {
                Ok(record_result) => record_result,
                Err(error) => { write_error(error); return; }
            };
            found = false;
        };
     
    }

    match csv_writer.flush() {
        Ok(writer) => writer,
        Err(_) => todo!(),
    };

    match open::with("basic.csv", "notepad") {
        Ok(open_result) => {
            if open_result.success() {
                println!("CSV file opened.");
            }
        },
        Err(error) => {
            println!("File not found -> Org.-Msg.: {}", error);
            false;
        }
    };

    confirm_with_enter();

    match delete("basic.csv") {
        Ok(_) => println!("CSV file deleted."),
        Err(_) => todo!(),
    };

}
