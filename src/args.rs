pub const UPDATE_AS_USER_ENTERED_FLAG: &str = "--GoogleUpdateAsUserEntered";

#[derive(Debug)]
pub struct Args {
    pub dsn: String,
    pub sql_file: String,
    pub sheet_id: Option<String>,
    pub sheet_range: Option<String>,
    pub targets_file: Option<String>,
    pub google_cert: String,
    pub google_update_as_user_entered: bool,
}

pub fn process_args(args: &Vec<String>) -> Result<Args, String> {
    let google_update_as_user_entered = args
        .iter()
        .any(|arg| arg.as_str() == UPDATE_AS_USER_ENTERED_FLAG);
    //the flag is optional and may appear in any position, so it is removed
    //before validating the positional arguments below
    let positional: Vec<&str> = args
        .iter()
        .map(String::as_str)
        .filter(|arg| *arg != UPDATE_AS_USER_ENTERED_FLAG)
        .collect();
    let arg_count = positional.len();

    if arg_count < 4 {
        return Err(format!(
            "Not enough arguments. Usage: {} <DSN> <sqlFile> <sheetID> <sheetRange> [googleCert] [--GoogleUpdateAsUserEntered]",
            args[0]
        ));
    } else if arg_count > 6 {
        return Err(format!(
            "Too many arguments. Usage: {} <DSN> <sqlFile> <sheetID> <sheetRange> [googleCert] [--GoogleUpdateAsUserEntered]",
            args[0]
        ));
    }

    let dsn = positional[1];
    let sql_file = positional[2];
    let sheet_id = Some(positional[3].to_string());
    //if sheet_id ends with .txt, then it is a targets file
    if sheet_id.as_ref().unwrap().ends_with(".txt") {
        let targets_file = sheet_id;

        let google_cert = if arg_count == 5 {
            positional[4]
        } else {
            "googleCert.json"
        };
        return Ok(Args {
            dsn: dsn.to_string(),
            sql_file: sql_file.to_string(),
            sheet_id: None,
            sheet_range: None,
            targets_file,
            google_cert: google_cert.to_string(),
            google_update_as_user_entered,
        });
    } else {
        let sheet_range = positional[4];
        let google_cert = if arg_count == 6 {
            positional[5]
        } else {
            "googleCert.json"
        };
        return Ok(Args {
            dsn: dsn.to_string(),
            sql_file: sql_file.to_string(),
            sheet_id,
            sheet_range: Some(sheet_range.to_string()),
            targets_file: None,
            google_cert: google_cert.to_string(),
            google_update_as_user_entered,
        });
    }
}
