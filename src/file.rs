pub mod json {
    use serde::Deserialize;
    use std::{error::Error, fs::{self, File}, io::BufReader};

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub cal_url: String,
    }
    pub fn read_config() -> Result<Config, Box<dyn Error>> {
        let filename = "config.json";
        let reader = BufReader::new(File::open(filename).unwrap());
        let config = serde_json::from_reader(reader).unwrap();
        return Ok(config);
    }

    pub fn delete (file:&str) -> std::io::Result<()> {
        fs::remove_file(file)?;
        Ok(())
    }
}