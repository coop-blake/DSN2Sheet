use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if (args.len() != 6) && (args.len() != 5) {
        println!(
            "Usage: {} <DSN> <sqlFile> <sheetID> <sheetRange> [googleCert]",
            args[0]
        );
        println!("You entered:> {}", args.join(" "));
        return;
    }

    let dsn = &args[1];
    let sql_file = &args[2];
    let sheet_id = &args[3];
    let sheet_range = &args[4];
    let google_cert = if args.len() > 5 {
        &args[5]
    } else {
        "googleCert.json"
    };

    // Print out the extracted arguments (for testing purposes)
    println!("DSN: {}", dsn);
    println!("SQL File: {}", sql_file);
    println!("Sheet ID: {}", sheet_id);
    println!("Sheet Range: {}", sheet_range);
    println!("Google Cert: {}", google_cert);
}
