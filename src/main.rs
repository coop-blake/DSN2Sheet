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
                    match odbc::get_data_from_dsn(&dsn, &query).await
                    {
                        Ok(data_for_google) => {
                            println!("Query executed successfully");
                            match args.targets_file {
                                Some(ref targets_file) => {
                                    println!("Uploading query result to targets");
                                    match read_sheet_data(targets_file)
                                    {
                                        Ok(targets) =>{
                                           let _ = send_to_multiple_targets(args.google_cert, data_for_google, targets).await;
                                        }
                                        Err(e) => {
                                            eprintln!("Error: {:?}", e)
                                        }
                                    }
                                }
                                None => {
                                    println!("Uploading to Sheet");
                                    match (&args.sheet_id, &args.sheet_range)
                                    {
                                        (Some(sheet_id), Some(sheet_range)) => {
                                            let target = vec![(sheet_id.clone(), sheet_range.clone())];
                                            let _ = send_to_multiple_targets(args.google_cert, data_for_google, target).await;
                                  
                                        }
                                        _ => {
                                            eprintln!("Unexecpected arguments:\n\t sheet_id:{:?} \n\tsheet_range:{:?}", args.sheet_id, args.sheet_range)
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
        Err(err) =>{
        //Argument Error
        eprintln!("{}", err);
        std::process::exit(1);   
     }
    }

}

use tokio::sync::Semaphore;
use tokio::task;
use std::sync::Arc;
use futures::future::join_all;

async fn send_to_multiple_targets(google_cert: String, data_for_google: Vec<Vec<String>>, targets: Vec<(String,String)>) -> Result<(), String> {
    let semaphore = Arc::new(Semaphore::new(1));
    let mut tasks = vec![];
    for (sheet_id, sheet_range) in targets {
        println!("Preparing to send to: {} @ {}", sheet_id, sheet_range);
        let semaphore = semaphore.clone();
        let google_cert = google_cert.clone();
        let data_for_google = data_for_google.clone();
        let task = task::spawn(async move {
            let permit = semaphore.acquire().await.unwrap();
            println!("Got Permit: {} @ {}", sheet_id, sheet_range);
            if let Err(err) = google::send_data_to_google_sheet(
                &google_cert,
                data_for_google.clone(),
                &sheet_id,
                &sheet_range,
            )
            .await
            {
                eprintln!("Error: {}", err)
            }
            drop(permit); // Release the permit when done
        });
        tasks.push(task);
    }

    let _ = join_all(tasks).await;
return Ok(())
}