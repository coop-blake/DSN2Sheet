use google_sheets4 as sheets4;
use hyper::Client;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde_json::Value;
use sheets4::api::{BatchUpdateValuesRequest, ValueRange};
use sheets4::oauth2::{self};
use sheets4::Sheets;
use sheets4::{hyper, hyper_rustls};

use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

pub async fn send_to_multiple_targets(
    google_cert: String,
    data_for_google: Vec<Vec<String>>,
    targets: Vec<(String, String)>,
) -> Result<(), String> {
    let semaphore = Arc::new(Semaphore::new(1));
    let mut tasks = vec![];
    for (sheet_id, sheet_range) in targets {
        // println!("Preparing to send to: {} @ {}", sheet_id, sheet_range);
        let semaphore = semaphore.clone();
        let google_cert = google_cert.clone();
        let data_for_google = data_for_google.clone();
        let task = task::spawn(async move {
            let permit = semaphore.acquire().await.unwrap();
            //  println!("Sending: {} @ {}", sheet_id, sheet_range);
            if let Err(err) = send_data_to_google_sheet(
                &google_cert,
                data_for_google.clone(),
                &sheet_id,
                &sheet_range,
            )
            .await
            {
                eprintln!("Error: {}", err)
            }
            // println!("Sent: {} @ {}", sheet_id, sheet_range);

            drop(permit); // Release the permit when done
        });
        tasks.push(task);
    }

    let _ = join_all(tasks).await;
    return Ok(());
}

pub async fn send_data_to_google_sheet(
    credentials_path: &str,
    data: Vec<Vec<String>>,
    spreadsheet_id: &str,
    range: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Build the HTTPS connector with desired configurations
    let https_connector = HttpsConnector::from(
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("no native root CA certificates found")
            .https_only() // Set HTTPS only
            .enable_http1() // Enable HTTP/1.x
            .enable_http2() // Enable HTTP/2
            .build(),
    );

    // Create a Hyper client with the HTTPS connector
    let client: Client<_, hyper::Body> = Client::builder().build(https_connector);

    // Initialize the Google Sheets API client
    let secret: oauth2::ServiceAccountKey = oauth2::read_service_account_key(&credentials_path)
        .await
        .expect("secret not found");
    // Create an authenticator
    let auth = oauth2::ServiceAccountAuthenticator::with_client(secret, client.clone())
        .build()
        .await
        .expect("could not create an authenticator");
    // Create a Sheets hub
    let hub = Sheets::new(client.clone(), auth);
    //convert the data to Vector of Vectors of Values
    let values_as_json: Vec<Vec<Value>> = data
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|item| Value::String(item.to_string())) // Convert &str to Value::String
                .collect()
        })
        .collect();

    // Create a ValueRange object from the Vector of Vectors
    let value_range = ValueRange {
        range: Some(range.to_string()), // Specify the range where data will be written
        major_dimension: Some("ROWS".to_string()),
        values: Some(values_as_json),
    };

    // Prepare the request to clear values
    let clear_request = sheets4::api::ClearValuesRequest::default();
    //execute the request to clear values from the spreadsheet
    let _ = hub
        .spreadsheets()
        .values_clear(clear_request, spreadsheet_id, range)
        .doit()
        .await?;

    // Prepare the request to append values
    let request = BatchUpdateValuesRequest {
        value_input_option: Some("RAW".to_string()), // How the input data should be interpreted
        data: Some(vec![value_range]),
        include_values_in_response: None,
        response_value_render_option: None,
        ..Default::default()
    };

    // Execute the request to append values to the spreadsheet
    let write_result = hub
        .spreadsheets()
        .values_batch_update(request, spreadsheet_id)
        .doit()
        .await?;

    //get sheet name from id
    let sheet = hub
        .spreadsheets()
        .get(spreadsheet_id)
        .doit()
        .await?
        .1
        .sheets
        .unwrap();

    let sheet_name = sheet
        .get(0)
        .unwrap()
        .properties
        .as_ref()
        .unwrap()
        .title
        .as_ref()
        .unwrap();
    //Get Spreedsheet document title
    let title = hub.spreadsheets().get(spreadsheet_id).doit().await?;
    let title = title.1.properties.as_ref().unwrap().title.as_ref().unwrap();

    //if write_result is successful, print the range where the data was written
    if let Some(updated_range) = write_result.1.responses {
        if let Some(returned_range) = &updated_range.get(0).unwrap().updated_range {
            println!(
                "Data written to {}; {} at range {}",
                title, sheet_name, returned_range
            );

            if returned_range
                .to_lowercase()
                .contains(&range.to_lowercase())
            {
                println!("Range Matches Input: {}", range);
                Ok(())
            } else {
                println!("Range Does Not Match Input: {}", range);
                Ok(())
            }
        } else {
            Err("Data was not written to the sheet".into())
        }
    } else {
        Err("Unexpected response from Google Sheets".into())
    }
}
