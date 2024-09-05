use dsn2_sheet::{args::process_args, file::read_sheet_data};
use std::env;
mod file;
mod google;
mod odbc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match process_args(&args) {
        Ok(args) => {
            let dsn = args.dsn;
            let sql_file = args.sql_file;
            println!("Reading SQL File");
            match file::get_string_from_file(&sql_file) {
                Ok(query) => {
                    println!("Executing Query");
                    match odbc::get_data_from_dsn(&dsn, &query).await {
                        Ok(data_for_google) => {
                            println!("Query executed successfully");
                            match args.targets_file {
                                Some(ref targets_file) => {
                                    println!("Uploading query result to targets");
                                    match read_sheet_data(targets_file) {
                                        Ok(targets) => {
                                            match google::send_to_multiple_targets(
                                                args.google_cert,
                                                data_for_google,
                                                targets,
                                            )
                                            .await
                                            {
                                                Ok(_) => println!("Data uploaded to Google Sheet"),
                                                Err(e) => eprintln!("Error: {:?}", e),
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Error: {:?}", e)
                                        }
                                    }
                                }
                                None => {
                                    println!("Uploading to Sheet");
                                    match (&args.sheet_id, &args.sheet_range) {
                                        (Some(sheet_id), Some(sheet_range)) => {
                                            match google::send_data_to_google_sheet(
                                                &args.google_cert,
                                                data_for_google,
                                                &sheet_id,
                                                &sheet_range,
                                            )
                                            .await
                                            {
                                                Ok(_) => println!("Data uploaded to Google Sheet"),
                                                Err(e) => eprintln!("Error: {:?}", e),
                                            }
                                        }
                                        _ => {
                                            eprintln!("Unexpected arguments:\n\t sheet_id:{:?} \n\tsheet_range:{:?}", args.sheet_id, args.sheet_range)
                                        }
                                    }
                                }
                            }
                        }
                        //Not able to get data from DSN
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                //Not able to get sql query from file
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(err) => {
            //Argument Error
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_args_with_sheet() {
        let args = vec![
            "test".to_string(),
            "dsn".to_string(),
            "sql_file".to_string(),
            "sheet_id".to_string(),
            "sheet_range".to_string(),
            "google_cert.json".to_string(),
        ];

        let args = process_args(&args).unwrap();
        assert_eq!(args.dsn, "dsn");
        assert_eq!(args.sql_file, "sql_file");
        assert_eq!(args.sheet_id, Some("sheet_id".to_string()));
        assert_eq!(args.sheet_range, Some("sheet_range".to_string()));
        assert_eq!(args.targets_file, None);
        assert_eq!(args.google_cert, "google_cert.json");
    }

    #[test]
    fn test_valid_args_with_targets_file() {
        let args = vec![
            "test".to_string(),
            "dsn".to_string(),
            "sql_file".to_string(),
            "targets_file.txt".to_string(),
        ];

        let result = process_args(&args).unwrap();
        assert_eq!(result.dsn, "dsn");
        assert_eq!(result.sql_file, "sql_file");
        assert_eq!(result.sheet_id, None);
        assert_eq!(result.sheet_range, None);
        assert_eq!(result.targets_file, Some("targets_file.txt".to_string()));
        assert_eq!(result.google_cert, "googleCert.json");
    }

    #[test]
    fn test_invalid_args_count() {
        let args = vec![
            "test".to_string(),
            "dsn".to_string(),
            "sql_file".to_string(),
        ];

        let result = process_args(&args);
        println!("result: {:?}", result);
        assert!(result.is_err());
        //assert_eq!(result.err().unwrap(), "Missing DSN");
    }
}
