use odbc_api::{buffers::TextRowSet, ConnectionOptions, Cursor, Environment, ResultSetMetadata};
use std::io::Write;

const BATCH_SIZE: usize = 5000;
use std::fs::File;

pub async fn get_data_from_dsn(
    dsn_name: &String,
    query_string: &String,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let file = File::create("output.txt")?;
    let mut fileWriter = csv::Writer::from_writer(file);

    let mut return_results: Vec<Vec<String>> = Vec::new();
    println!("Getting Data from \x1b[34m{}\x1b[0m ", dsn_name);

    println!("\x1b[34mQuery: {}\x1b[0m", query_string);

    let environment = Environment::new()?;

    let mut odbc_connection =
        environment.connect(dsn_name, "", "", ConnectionOptions::default())?;

    match odbc_connection.execute(query_string, ())? {
        Some(mut cursor) => {
            println!("Data received");
            let mut header: Vec<String> = cursor.column_names()?.collect::<Result<_, _>>()?;

            return_results.push(header.clone());
            fileWriter.write_record(header.clone())?;

            let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;

            let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

            while let Some(batch) = row_set_cursor.fetch()? {
                for row_index in 0..batch.num_rows() {
                    let record: Vec<String> = (0..batch.num_cols())
                        .map(|col_index| match batch.at(col_index, row_index) {
                            Some(bytes) => match String::from_utf8(bytes.to_vec()) {
                                Ok(s) => s,
                                Err(_) => String::from("Invalid UTF-8"),
                            },
                            None => String::from("Missing value"),
                        })
                        .collect();

                    return_results.push(record.clone());

                    fileWriter.write_record(record.clone())?;
                }
            }
        }
        None => {
            eprintln!("Query came back empty. No output has been created.");
        }
    }

    Ok(return_results)
}
