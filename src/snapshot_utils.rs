use crate::configs::load_config;
use aws_sdk_ec2::{Client, Error};
use chrono::{DateTime, Duration, Utc};
use std::io::Error as StdError;

pub async fn prepare_snapshots_for_deletion_as_array() -> Result<Vec<(String, i64, String)>, Error>
{
    let existing_snapshots = fetch_existing_snapshots_as_array().await;
    let snapshot_ids_for_deletion = existing_snapshots?;

    Ok(snapshot_ids_for_deletion)
}

#[allow(dead_code)]
pub async fn fetch_existing_snapshots_as_array() -> Result<Vec<(String, i64, String)>, Error> {
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

    count_snapshots().await?;

    let mut snapshot_ids = Vec::new();

    // snapshot_ids produces the following,
    // need to conver the second field to something more useful...
    // - Snapshot ID: "snap-02ee1dfe2efa1a54a", Start Time: "2024-02-06T04:08:04.65Z"
    //- Snapshot ID: "snap-09b40966263b9e763", Start Time: "2024-02-06T00:00:21.185Z"
    //

    for snapshot in snapshots {
        if let (Some(snapshot_id), Some(start_time)) =
            (snapshot.snapshot_id(), snapshot.start_time())
        {
            // Convert &str to String for easier manipulation
            let start_time_str = start_time.to_string();
            if let Ok(start_time_dt) = DateTime::parse_from_rfc3339(&start_time_str) {
                // Convert to UTC timezone
                let start_time_utc = start_time_dt.with_timezone(&Utc);
                // Convert to seconds since Unix epoch
                let start_time_seconds = start_time_utc.timestamp();
                snapshot_ids.push((snapshot_id.to_string(), start_time_seconds, start_time_str));
            } else {
                eprintln!("Error parsing start time: {}", start_time);
            }
        }
    }

    Ok(snapshot_ids)
}

#[allow(dead_code)]
pub async fn older_than(_days: String) -> Result<String, StdError> {
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

    // 1707275144 - 259200 = last_timestamp_to_delete?
    // 2592000

    // Use the chrono lib to get the current system timestamp.
    // 1707275144
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    eprintln!("Chrono UTC Timestamp: {:?}", timestamp);

    let specified_time = Duration::days(30).num_seconds().to_string();

    eprintln!("Days: {:?}", specified_time);

    Ok(specified_time)
}

pub async fn count_snapshots() -> Result<(), Error> {
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

#[allow(dead_code)]
pub async fn delete_snapshots() -> Result<(), Error> {
    // snapshotd --delete-if-older-than 30
    Ok(())
}
