use crate::configs::load_config;
use aws_sdk_ec2::{Client, Error};
use dotenv::dotenv;

#[allow(dead_code)]
pub async fn fetch_existing_snapshots() -> Result<(), Error> {
    dotenv().ok();
    //let env_region = "REGION";

    let config = match load_config().await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading AWS SDK configuration: {:?}", e);
            return Err(e); // Convert the error type if necessary
        }
    };

    let ec2 = Client::new(&config);

    let resp = ec2.describe_snapshots().owner_ids("self").send().await?;
    let snapshots = resp.snapshots();
    let length = snapshots.len();

    for snapshot in snapshots {
        println!(
            "ID:          {}",
            snapshot.snapshot_id().unwrap_or_default()
        );
        println!(
            "Description: {}",
            snapshot.description().unwrap_or_default()
        );
        println!("State:       {}", snapshot.state().unwrap().as_ref());
        println!();
    }

    println!();
    println!("Found {} snapshot(s)", length);
    println!();

    Ok(())
}

pub async fn fetch_existing_snapshots_as_array() -> Result<Vec<String>, Error> {
    let config = match load_config().await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading AWS SDK configuration: {:?}", e);
            return Err(e); // Convert the error type if necessary
        }
    };

    let ec2 = Client::new(&config);

    let resp = ec2.describe_snapshots().owner_ids("self").send().await?;
    let snapshots = resp.snapshots();

    let mut snapshot_ids = Vec::new();

    for snapshot in snapshots {
        if let Some(snapshot_id) = snapshot.snapshot_id() {
            snapshot_ids.push(snapshot_id.to_string());
        }
    }
    Ok(snapshot_ids)
}

#[allow(dead_code)]
pub async fn count_snapshots() -> Result<(), Error> {
    dotenv().ok();
    //let env_region = "REGION";

    let config = match load_config().await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading AWS SDK configuration: {:?}", e);
            return Err(e); // Convert the error type if necessary
        }
    };

    let ec2 = Client::new(&config);

    let resp = ec2.describe_snapshots().owner_ids("self").send().await?;
    let snapshots = resp.snapshots();
    let length = snapshots.len();

    println!("Found {} snapshot(s)", length);

    Ok(())
}
