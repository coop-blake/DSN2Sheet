use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
//function returns string from file given file name or error
pub fn get_string_from_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut result = String::new();
    for line in reader.lines() {
        //unwrap the line and add it to the result with a newline
        result.push_str(&line?);
        result.push('\n');
    }
    //remove the last newline
    result.pop();
    Ok(result)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MyError {
    IoError(io::Error),
    InvalidFormat,
    EmptyFile,
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::IoError(error)
    }
}
#[allow(dead_code)]
pub fn read_sheet_data(filename: &str) -> Result<Vec<(String, String)>, MyError> {
    let path = Path::new(filename);
    // println!(
    //     "Path: {:?} Filename: {:?}",
    //     path.to_str().unwrap(),
    //     filename
    // );

    let file = File::open(&path)?;

    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('@').collect();
        if parts.len() != 2 {
            return Err(MyError::InvalidFormat);
        }
        data.push((parts[0].to_string(), parts[1].to_string()));
    }

    if data.is_empty() {
        return Err(MyError::EmptyFile);
    }
    Ok(data)
}
