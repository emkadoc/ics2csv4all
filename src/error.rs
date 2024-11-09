pub mod csv {
    pub fn write_error(error: csv::Error) -> () {
        println!("Fehler. Schreiben in CSV-File nicht möglich. -> Error-Msg: {})", error);
    }
}
pub mod ui {
    use chrono::ParseError;

    pub fn parse_error(error: ParseError) -> () {
        println!("Eingabefehler. Bitte Schema beachten (Schema: JJJJ-MM-TT) -> Orig.-Msg.: {}", error);
    }
}