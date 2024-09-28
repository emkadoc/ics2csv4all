pub mod json {
    use serde::Deserialize;
    use std::{error::Error, fs::File, io::BufReader};

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub cal_url: String,
    }
    pub fn read_config() -> Result<Config, Box<dyn Error>> {
        let reader = BufReader::new(File::open("config.json").unwrap());
        let config = serde_json::from_reader(reader).unwrap();
        return Ok(config);
    }
}