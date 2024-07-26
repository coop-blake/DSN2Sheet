use std::env::{self, var};


#[derive(Debug)]
pub struct Args {
   pub dsn: String,
   pub sql_file: String,
   pub sheet_id: Option<String>,
   pub sheet_range: Option<String>,
   pub targets_file: Option<String>,
   pub google_cert: String,
}

pub fn process_args(args: &Vec<String>) -> Result<Args, String> {
    if (args.len() != 5) && (args.len() != 6) && (args.len() != 4) {

        return Err(format!(
            "Usage: {} <DSN> <sqlFile> <sheetID> <sheetRange> [googleCert]",
            args[0]
        ));
    }

    let dsn = &args[1];
    let sql_file = &args[2];
    let mut sheet_id: Option<String> = None  ;
    let mut sheet_range: Option<String> = None ;
    let targets_file: Option<String> = None;


    let google_cert = if args.len() > 4 {
        &args[4]
    } else {
        "googleCert.json"
    };

    if args.len() == 5 {
        // Handle the case with sheetID and sheetRange
        sheet_id = Some(args[3].clone());

        sheet_range = Some(args[4].clone());
        
        // Print out the extracted arguments (for testing purposes)
        println!("DSN: {}", dsn);
        println!("SQL File: {}", sql_file);
        println!("Sheet ID: {:?}", sheet_id);
        println!("Sheet Range: {:?}", sheet_range);
        println!("Google Cert: {}", google_cert);
    } else if args.len() == 4 {
        // Handle the case with targetsFile
        let targets_file = &args[3];
        
        // Print out the extracted arguments (for testing purposes)
        println!("DSN: {}", dsn);
        println!("SQL File: {}", sql_file);
        println!("Targets File: {}", targets_file);
        println!("Google Cert: {}", google_cert);
    }

    println!("Reading SQL File");

    Ok(Args {
        dsn: dsn.to_string() ,
        sql_file: sql_file.to_string(),
        sheet_id:sheet_id,
        sheet_range:sheet_range,
        targets_file:targets_file,
        google_cert:google_cert.to_string(),
    })
}

