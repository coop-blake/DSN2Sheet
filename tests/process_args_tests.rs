use dsn2_sheet::args::process_args;

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
        assert!(!args.google_update_as_user_entered);
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
        assert!(!result.google_update_as_user_entered);
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

    #[test]
    fn test_valid_args_with_sheet_and_user_entered_flag() {
        let args = vec![
            "test".to_string(),
            "dsn".to_string(),
            "sql_file".to_string(),
            "sheet_id".to_string(),
            "sheet_range".to_string(),
            "--GoogleUpdateAsUserEntered".to_string(),
        ];

        let args = process_args(&args).unwrap();
        assert_eq!(args.dsn, "dsn");
        assert_eq!(args.sql_file, "sql_file");
        assert_eq!(args.sheet_id, Some("sheet_id".to_string()));
        assert_eq!(args.sheet_range, Some("sheet_range".to_string()));
        assert_eq!(args.targets_file, None);
        assert_eq!(args.google_cert, "googleCert.json");
        assert!(args.google_update_as_user_entered);
    }

    #[test]
    fn test_valid_args_with_targets_file_and_user_entered_flag() {
        let args = vec![
            "test".to_string(),
            "dsn".to_string(),
            "sql_file".to_string(),
            "targets_file.txt".to_string(),
            "--GoogleUpdateAsUserEntered".to_string(),
        ];

        let result = process_args(&args).unwrap();
        assert_eq!(result.dsn, "dsn");
        assert_eq!(result.sql_file, "sql_file");
        assert_eq!(result.sheet_id, None);
        assert_eq!(result.sheet_range, None);
        assert_eq!(result.targets_file, Some("targets_file.txt".to_string()));
        assert_eq!(result.google_cert, "googleCert.json");
        assert!(result.google_update_as_user_entered);
    }

    #[test]
    fn test_valid_args_with_cert_and_user_entered_flag() {
        //the flag may appear in any position, including after a custom certificate
        let args = vec![
            "test".to_string(),
            "dsn".to_string(),
            "sql_file".to_string(),
            "sheet_id".to_string(),
            "sheet_range".to_string(),
            "google_cert.json".to_string(),
            "--GoogleUpdateAsUserEntered".to_string(),
        ];

        let args = process_args(&args).unwrap();
        assert_eq!(args.sheet_id, Some("sheet_id".to_string()));
        assert_eq!(args.sheet_range, Some("sheet_range".to_string()));
        assert_eq!(args.google_cert, "google_cert.json");
        assert!(args.google_update_as_user_entered);
    }

    #[test]
    fn test_valid_args_with_user_entered_flag_in_first_position() {
        //the flag is recognized no matter where it appears in the arguments
        let args = vec![
            "test".to_string(),
            "--GoogleUpdateAsUserEntered".to_string(),
            "dsn".to_string(),
            "sql_file".to_string(),
            "targets_file.txt".to_string(),
            "google_cert.json".to_string(),
        ];

        let result = process_args(&args).unwrap();
        assert_eq!(result.dsn, "dsn");
        assert_eq!(result.sql_file, "sql_file");
        assert_eq!(result.targets_file, Some("targets_file.txt".to_string()));
        assert_eq!(result.google_cert, "google_cert.json");
        assert!(result.google_update_as_user_entered);
    }

    #[test]
    fn test_duplicate_user_entered_flags_treated_as_one() {
        let args = vec![
            "test".to_string(),
            "dsn".to_string(),
            "sql_file".to_string(),
            "targets_file.txt".to_string(),
            "--GoogleUpdateAsUserEntered".to_string(),
            "--GoogleUpdateAsUserEntered".to_string(),
        ];

        let result = process_args(&args).unwrap();
        assert_eq!(result.targets_file, Some("targets_file.txt".to_string()));
        assert!(result.google_update_as_user_entered);
    }

    #[test]
    fn test_flag_only_without_positional_arguments_is_error() {
        let args = vec![
            "test".to_string(),
            "--GoogleUpdateAsUserEntered".to_string(),
        ];

        let result = process_args(&args);
        assert!(result.is_err());
    }
}
