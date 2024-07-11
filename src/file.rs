use std::{
    fs::File,
    io::{BufRead, BufReader},
};

//function returns string from file given file name or erro
pub fn get_string_from_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut result = String::new();
    for line in reader.lines() {
        result.push_str(&line?);
    }
    Ok(result)
}
