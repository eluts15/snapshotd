use crate::configs::load_config;
use aws_sdk_ec2::{Client, Error};
use chrono::{DateTime, Duration, Utc};

// snapshotd --delete-if-older-than 30

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
// Redundant as fuck..
pub async fn fetch_existing_snapshots_timestamps() -> Result<Vec<i64>, Error> {
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

    let mut snapshot_timstamps = Vec::new();

    for snapshot in snapshots {
        if let Some(start_time) = snapshot.start_time() {
            // Convert &str to String for easier manipulation
            let start_time_str = start_time.to_string();
            if let Ok(start_time_dt) = DateTime::parse_from_rfc3339(&start_time_str) {
                // Convert to UTC timezone
                let start_time_utc = start_time_dt.with_timezone(&Utc);
                // Convert to seconds since Unix epoch
                let start_time_seconds = start_time_utc.timestamp();
                snapshot_timstamps.push(start_time_seconds);
            } else {
                eprintln!("Error parsing start time: {:?}", start_time);
            }
        }
    }

    Ok(snapshot_timstamps)
}

#[allow(dead_code)]
// should accept a number of days, and return the ids, that are older than the day provided.
// find a mechanism for mapping the IDs to timestamps
pub async fn older_than(_days: i64) -> Result<(), Error> {
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    //eprintln!("Chrono UTC Timestamp: {:?}", timestamp);

    let specified_time = Duration::days(30).num_seconds(); //bullshit

    let older_than_thirty_days = timestamp - specified_time;

    // Created At: 1707192484
    // Created At: 1707177621
    let snapshot_timestamps = fetch_existing_snapshots_timestamps().await?;

    let mut snapshots_to_delete = Vec::new();

    for timestamp in snapshot_timestamps {
        if timestamp > older_than_thirty_days {
            snapshots_to_delete.push(timestamp);

            eprintln!(
                "The following are older than x days and will be delete: {:?}",
                snapshots_to_delete,
            );
        } else {
            println!("Not found");
        }
    }
    Ok(())
}

#[allow(dead_code)]
// return the IDs, iterate, and finally delete them.
pub async fn delete_snapshots() -> Result<(), Error> {
    let config = match load_config().await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading AWS SDK configuration: {:?}", e);
            return Err(e); // Convert the error type if necessary
        }
    };

    let ec2 = Client::new(&config);

    older_than(123213).await?;
    ec2.delete_snapshot();
    Ok(())
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
