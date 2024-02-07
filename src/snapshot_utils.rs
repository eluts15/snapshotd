use crate::configs::load_config;
use aws_sdk_ec2::{Client, Error};
use chrono::prelude::*;
use dotenv::dotenv;
use std::time::{SystemTime, UNIX_EPOCH};

// Defaults to deleting snapshots that are over 30 days old. (Need to verify how AWS dates
// things...)
// Optionally, should be able to specify a number of days from now -- older will be deleted.
pub async fn prepare_snapshots_for_deletion_as_array() -> Result<Vec<String>, Error> {
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

    let mut snapshot_ids_for_deletion = Vec::new();

    // Get the current time
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH);
    //2024-02-06T23:24:39.750383Z`
    //TODO: Convert this to UNIX timestamp
    eprintln!("Current SystemTime UNIX: {:?}", current_time);

    if let Some(aws_snap_time) = snapshots.first().and_then(|snapshot| snapshot.start_time()) {
        let epoch_seconds = aws_snap_time.secs();

        eprintln!("AWS Snaphot Time UNIX: {:?}", epoch_seconds);
    }

    // Calculate the timestamp one day ago
    //let one_day_ago = current_time; //- Duration::days(7);
    //for snapshot in snapshots {
    //    if let Some(create_time) = snapshot.start_time() {
    //        // Compare timestamps
    //        if create_time < &current_time && create_time >= &one_day_ago {
    //            if let Some(snapshot_id) = snapshot.snapshot_id() {
    //                snapshot_ids_for_deletion.push(snapshot_id.to_string());
    //            }
    //        }
    //    }
    //}

    Ok(snapshot_ids_for_deletion)
}

///
/// Helper Functions...
///
#[allow(dead_code)]
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

        todo!("Get StartTime of the snapshot for comparision later..");
    }

    println!();
    println!("Found {} snapshot(s)", length);
    println!();

    Ok(())
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
