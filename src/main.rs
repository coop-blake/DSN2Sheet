use std::env;
mod colors;
mod file;
mod google;
mod odbc;
fn main() {
    colors::write_blue();
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

    println!("Starting");
    colors::write_none();

    let mut query = String::new();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let sql_results = file::get_string_from_file(sql_file);
        match sql_results {
            Ok(results) => {
                println!("String from File Received:");
                query = results;
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    if query.is_empty() {
        eprintln!("Query is empty. Exiting.");
        return;
    }
    let mut data_for_google: Option<Vec<Vec<String>>> = None;

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let query_results = odbc::get_data_from_dsn(dsn, &query).await;
        match query_results {
            Ok(results) => {
                data_for_google = Some(results);
                println!("Query executed successfully")
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    });

    if data_for_google.is_some() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            println!("Preparing to send data to Google Sheets");
            if let Err(err) = google::send_data_to_google_sheet(
                &google_cert,
                data_for_google.unwrap(),
                &sheet_id,
                &sheet_range,
            )
            .await
            {
                eprintln!("Error: {}", err)
            }
        });
    } else {
        eprintln!("No data received from DSN. Exiting.");
        return;
    }

    println!("Ending");
}
