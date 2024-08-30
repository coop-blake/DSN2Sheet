
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
    let arg_count = args.len();

    if arg_count < 4 {
        return Err(format!(
            "Not enough arguments. Usage: {} <DSN> <sqlFile> <sheetID> <sheetRange> [googleCert]",
            args[0]
        ));
    } else if arg_count > 6 {
        return Err(format!(
            "Too many arguments. Usage: {} <DSN> <sqlFile> <sheetID> <sheetRange> [googleCert]",
            args[0]
        ));
    }

    let dsn = &args[1];
    let sql_file = &args[2];
    let sheet_id = Some(args[3].clone());
    //if sheet_id ends with .txt, then it is a targets file
    if sheet_id.as_ref().unwrap().ends_with(".txt") {
        let targets_file = sheet_id.clone();

        let google_cert = if arg_count == 5 {
            &args[4]
        } else {
            "googleCert.json"
        };
        return Ok(Args {
            dsn: dsn.to_string(),
            sql_file: sql_file.to_string(),
            sheet_id: None,
            sheet_range: None,
            targets_file: targets_file,
            google_cert: google_cert.to_string(),
        });
    } else {
        let sheet_range = &args[4];
        let google_cert = if arg_count == 6 {
            &args[5]
        } else {
            "googleCert.json"
        };
        return Ok(Args {
            dsn: dsn.to_string(),
            sql_file: sql_file.to_string(),
            sheet_id: sheet_id,
            sheet_range: Some(sheet_range.to_string()),
            targets_file: None,
            google_cert: google_cert.to_string(),
        });
    }
}
