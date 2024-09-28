pub mod csv {
    pub fn write_error(error: csv::Error) -> () {
        println!("Fehler. Schreiben in CSV-File nicht möglich. -> Error-Msg: {})", error);
    }
}
pub mod ui {
    use std::io;
    use chrono::ParseError;

    pub fn parse_error(error: ParseError) -> () {
        println!("Eingabefehler. Bitte Schema beachten (Schema: JJJJ-MM-TT) -> Orig.-Msg.: {}", error);
        println!("Bitte Eingabetaste betätigen um Fenster zu schließen.");
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);
        let _ = result.is_ok();
    }
}