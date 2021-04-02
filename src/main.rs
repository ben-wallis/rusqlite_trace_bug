use rusqlite::Connection;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SqlLogMode {
    /// Logging is disabled
    Disabled,
    /// Records timings for each SQL statement
    Profile,
    /// Prints all executed SQL statements
    Trace,
}

fn main() {
    let mut connection = Connection::open(r#"C:\rust\veloren3\userdata\server\saves\db.sqlite"#).unwrap();

    set_log_mode(&mut connection, SqlLogMode::Trace);

    connection.pragma_update(None, "foreign_keys", &"ON").unwrap();
}

fn set_log_mode(connection: &mut Connection, sql_log_mode: SqlLogMode) {

    match sql_log_mode {
        SqlLogMode::Trace => {
            println!("setting trace");
            connection.trace(Some(rusqlite_trace_callback));
            connection.profile(None);
        },
        SqlLogMode::Profile => {
            // connection.profile(Some(rusqlite_profile_callback));
            // connection.trace(None);
        },
        _ => {
            connection.trace(None);
            connection.profile(None);
        },
    };
}

fn rusqlite_trace_callback(log_message: &str) {
    println!("{}", log_message);
}
