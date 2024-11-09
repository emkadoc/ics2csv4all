# About "ics2csv4win"
This is an exporter for online ics calendars written in Rust[^1]. 
It downloads all appointments by the given time period and transfers the results into character/tab separated values (csv).

# Background | Intention
This is the Rust-Implementation of the older plain Windows Batch-Exporter [ics2csv4win](https://github.com/emkadoc/ics2csv4win).

# Configuration
1. Download executable file from [Releases-Page](https://github.com/emkadoc/ics2csv4all/releases)
1. Create a configuration file named _**config.json**_ with the following initial content:
   ```json
   {
    "cal_url" : "https://<domain>/<path_to_ics_file>"
   }
   ```
1. Place the _**config.json**_ beside the executable file

# Usage
1. Start the executable by double-klick executable file or run inside the terminal
1. Please enter the time period to be exported by
   1. Entering the start date and
   2. Entering the end date
1. The online ics will be downloaded and the results will be presented in the systems editor
1. You can copy the results e.g. into excel for further processing

# Miscellaneous 

[^1]: https://www.rust-lang.org/
