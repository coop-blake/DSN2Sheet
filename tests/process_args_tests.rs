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
        assert!(result.is_err());
        //assert_eq!(result.err().unwrap(), "Missing DSN");

    }

    #[test]
    fn test_missing_dsn() {
        let args = vec![
            "test".to_string(),
            "sql_file".to_string(),
            "sheet_id".to_string(),
            "sheet_range".to_string(),
            "google_cert.json".to_string(),
        ];

        let result = process_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_sql_file() {
        let args = vec![
            "test".to_string(),
            "dsn".to_string(),
            "sheet_id".to_string(),
            "sheet_range".to_string(),
            "google_cert.json".to_string(),
        ];

        let result = process_args(&args);
        assert!(result.is_err());
    }
}
