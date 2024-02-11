use std::collections::HashMap;
use std::error::Error as StdError;
use std::fs::File;
use csv;
use lambda_http::{run, service_fn, Body, Request, Response};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing::log::info;
use std::env;
use std::fs;

async fn function_handler(_: Request) -> Result<Response<Body>, Box<dyn StdError>> {
    // Specify the file name of your CSV file
    let file_name = "hotalingcocktailsCocktails.csv";

    // Log the current directory
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    
    let paths = fs::read_dir("./")?;
    for path in paths {
        println!("Name: {}", path?.path().display());
    }


    // Open the CSV file
    let file = File::open(file_name)?;
    let mut csv_reader = csv::Reader::from_reader(file);


    // Create a HashMap to count occurrences of each location
    let mut location_counts: HashMap<String, usize> = HashMap::new();

    let location_index = match csv_reader.headers()?.iter().position(|h| h == "Location") {
        Some(index) => index,
        None => {
            eprintln!("No 'Location' column found in the CSV file header.");
            return Ok(Response::builder()
                .status(500)
                .body("Internal Server Error".to_string().into())? // Convert to Body type
            );
        }
    };
    
    // Iterate over each record in the CSV file
    for result in csv_reader.records() {
        let record = result?;
    
        // Get the value of the "Location" column
        let location = match record.get(location_index) {
            Some(location) if !location.is_empty() => location.to_owned(),
            _ => {
                // Skip records with missing or empty location data
                continue;
            }
        };
        // Increment the count for the location in the HashMap
        *location_counts.entry(location).or_insert(0) += 1;
    }

    // Find the most common entry
    let most_common_entry = location_counts.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(location, _)| location);

    // Prepare response
    let response_body = match most_common_entry {
        Some(location) => format!("The most common location is: {}", location),
        None => "No location data found in the CSV file.".to_string(),
    };

    info!("The most common location is:");

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(response_body.into())? // Convert to Body type
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
    println!("Starting the server");
    run(service_fn(function_handler)).await?;
    Ok(())
}
