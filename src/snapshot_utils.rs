use crate::configs::load_config;
use aws_sdk_ec2::{Client, Error};
use chrono::{Duration, Utc};
use dotenv::dotenv;

#[allow(dead_code)]
pub async fn delete_if_older_than() -> Result<(), Error> {
    // Use the chrono lib to get the current system timestamp.
    // 1707275144
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    eprintln!("Chrono UTC Timestamp: {:?}", timestamp);

    // snapshotd --delete-if-older-than 30

    // consider the ideal data type for user String, i32, u8?
    // internally, it might be converting a String to an int of some type.

    // input 30 days (2592000)
    // 1) There's current_time (when application is run)
    // 2) There's the creation_time (the time when the snapshot was provisioned)
    // 3)
    //
    // current_time > creation_time, this should always be true.
    //
    // creation_time - time_in_days_specified

    // Calculate the timestamp 30 days ago, i guess this is where UNIX timestamp comes in..
    // current_time - specified_time = last_relevant_timestamp
    //
    // if time_stamp_to_delete >= last_relevant_timestamp
    // println(done)

    let specified_time = Duration::days(30).num_seconds(); // 2592000                                                           // 1707275144 - 259200 = last_timestamp_to_delete?
    eprintln!("Days: {:?}", dt);

    Ok(())
}

#[allow(dead_code)]
pub async fn prepare_snapshots_for_deletion_as_array() -> Result<Vec<(String, String)>, Error> {
    // Use the chrono lib to get the current system timestamp.
    // 1707275144
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    eprintln!("Chrono UTC Timestamp: {:?}", timestamp);

    let existing_snapshots = fetch_existing_snapshots_as_array().await;
    let snapshot_ids_for_deletion = existing_snapshots?;

    Ok(snapshot_ids_for_deletion)
}

///
/// Helper Functions...
///
#[allow(dead_code)]
pub async fn fetch_existing_snapshots_as_array() -> Result<Vec<(String, String)>, Error> {
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
        if let (Some(snapshot_id), Some(start_time)) =
            (snapshot.snapshot_id(), snapshot.start_time())
        {
            snapshot_ids.push((snapshot_id.to_string(), start_time.to_string()));
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
