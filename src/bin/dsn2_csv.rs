use dsn2_sheet::file::get_string_from_file;
use dsn2_sheet::odbc::write_data_from_dsn_to_file;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if (args.len() != 5) && (args.len() != 4) {
        println!("Usage: {} <DSN> <sqlFile> <filename> [googleCert]", args[0]);
        println!("You entered:> {}", args.join(" "));
        return;
    }

    let dsn = &args[1];
    let sql_file = &args[2];
    let filename = &args[3];
    let google_cert = if args.len() > 4 {
        &args[4]
    } else {
        "googleCert.json"
    };

    // Print out the extracted arguments (for testing purposes)
    println!("DSN: {}", dsn);
    println!("SQL File: {}", sql_file);
    println!("Filename: {}", filename);
    println!("Google Cert: {}", google_cert);

    println!("Reading SQL File");

    let mut query = String::new();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let sql_results = get_string_from_file(sql_file);
        match sql_results {
            Ok(results) => {
                query = results;
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    if query.is_empty() {
        eprintln!("Query is empty. Exiting.");
        return;
    }

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let query_results = write_data_from_dsn_to_file(dsn, &query, &filename).await;
        match query_results {
            Ok(()) => {
                println!("Data written to file successfully")
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    });

    println!("Ending");
}
