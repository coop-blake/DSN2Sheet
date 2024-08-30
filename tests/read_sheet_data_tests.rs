use dsn2_sheet::file::{read_sheet_data, MyError};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn write_test_file(content: &str, filename: &str) -> PathBuf {
    let  path = Path::new("tests").join("test_files").join(filename);

    let mut file = File::create(&path).expect("Unable to create test file");
    let _ = file.write_all(content.as_bytes());

    // Print the full path to the file
    let full_path = path.canonicalize().expect("Unable to get full path");
    println!("Test file created at: {}", full_path.display());

    path
}

#[test]
fn test_valid_data() {
    let filename = "test_valid_data.txt";
    let content = "sheet_id_1@range_1\nsheet_id_2@range_2\n";
    let path = write_test_file(content, filename);
    let filename = path.to_str().unwrap();
    println!("Path: {:?}", path.to_str().unwrap());
    let result = read_sheet_data(filename);
    let actual = result.unwrap();
    let expected = vec![
        ("sheet_id_1".to_string(), "range_1".to_string()),
        ("sheet_id_2".to_string(), "range_2".to_string())
    ];
    
    // Check that the length of actual and expected are the same
    assert_eq!(actual.len(), expected.len(), "The number of elements does not match");
    
    // Assert each pair individually
    for (i, (expected_item, actual_item)) in expected.into_iter().zip(actual).enumerate() {
        assert_eq!(expected_item, actual_item, "Mismatch at index {}", i);
    }
    std::fs::remove_file(filename).unwrap(); // Cleanup
}
#[test]
fn test_valid_data_with_spaces() {
    let filename = "test_valid_data_with_spaces.txt";
    let content = "sheet_id_1@range_1!A2:D\nsheet_id_2@range 2!A2:D\n";
    let path = write_test_file(content, filename);
    let filename = path.to_str().unwrap();
    println!("Path: {:?}", path.to_str().unwrap());
    let result = read_sheet_data(filename);
    assert_eq!(
        result.unwrap(),
        vec![
            ("sheet_id_1".to_string(), "range_1!A2:D".to_string()),
            ("sheet_id_2".to_string(), "range 2!A2:D".to_string())
        ]
    );
    std::fs::remove_file(filename).unwrap(); // Cleanup
}

#[test]
fn test_invalid_format() {
    let filename = "test_invalid_format.txt";
    let content = "sheet_id_1@range_1\ninvalid_format\n";
    let path = write_test_file(content, filename);
    let filename = path.to_str().unwrap();

    let result = read_sheet_data(filename);
    assert!(matches!(result, Err(MyError::InvalidFormat)));
    std::fs::remove_file(filename).unwrap(); // Cleanup
}

#[test]
fn test_empty_file() {
    let filename = "test_empty_file.txt";
    let content = "";
    let path = write_test_file(content, filename);
    let filename = path.to_str().unwrap();

    let result = read_sheet_data(filename);
    assert!(matches!(result, Err(MyError::EmptyFile)));
    std::fs::remove_file(filename).unwrap(); // Cleanup
}

#[test]
fn test_file_not_found() {
    let filename = "non_existent_file.txt";
    let result = read_sheet_data(filename);
    assert!(matches!(result, Err(MyError::IoError(_))));
}
