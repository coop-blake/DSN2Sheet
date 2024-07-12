use odbc_api::{buffers::TextRowSet, ConnectionOptions, Cursor, Environment, ResultSetMetadata};

const BATCH_SIZE: usize = 5000;
use std::fs::File;

use encoding_rs::WINDOWS_1252;

pub async fn get_data_from_dsn(
    dsn_name: &String,
    query_string: &String,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let file = File::create("output.txt")?;
    let mut file_writer = csv::Writer::from_writer(file);

    let mut return_results: Vec<Vec<String>> = Vec::new();
    println!("Getting Data from \x1b[34m{}\x1b[0m ", dsn_name);

    println!("\x1b[34mQuery: {}\x1b[0m", query_string);

    let environment = Environment::new()?;

    let odbc_connection = environment.connect(dsn_name, "", "", ConnectionOptions::default())?;

    match odbc_connection.execute(query_string, ())? {
        Some(mut cursor) => {
            println!("Data received");
            let header: Vec<String> = cursor.column_names()?.collect::<Result<_, _>>()?;

            return_results.push(header.clone());
            file_writer.write_record(header.clone())?;

            //set the max_str_limit to 50000 which is the maximum length of a string that can be stored in a google sheet cell
            let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(50000))?;

            let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

            while let Some(batch) = row_set_cursor.fetch()? {
                for row_index in 0..batch.num_rows() {
                    let record: Vec<String> = (0..batch.num_cols())
                        .map(|col_index| match batch.at(col_index, row_index) {
                            Some(bytes) => {
                                // Convert bytes using encoding_rs
                                let encoding = WINDOWS_1252;
                                let (result, _, _) = encoding.decode(bytes);
                                result.to_string()
                            }
                            None => String::from(""),
                        })
                        .collect();

                    return_results.push(record.clone());

                    file_writer.write_record(record.clone())?;
                }
            }
        }
        None => {
            eprintln!("Query came back empty. No output has been created.");
        }
    }
    // print_lengths(&return_results);

    Ok(return_results)
}

//function that prints the length of each string in  a vector of vectors of strings
pub fn print_lengths(data: &Vec<Vec<String>>) -> () {
    for row in data {
        for item in row {
            println!("{} ", item.len());
        }
        println!("{} ", row.len());
    }
}
