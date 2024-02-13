use aws_sdk_ec2::types::Snapshot;
use aws_sdk_ec2::{Client, Error};
use chrono::Utc;

pub fn process_snapshot(_client: &Client, snapshot: Snapshot) -> Vec<String> {
    // Extract snapshot ID
    let snapshot_id = match snapshot.snapshot_id {
        Some(id) => id,
        None => {
            eprintln!("Snapshot ID not found");
            return Vec::new();
        }
    };

    // Extract snapshot start time
    let start_time = match snapshot.start_time {
        Some(time) => time.secs(),
        None => {
            eprintln!("Snapshot start time not found");
            return Vec::new();
        }
    };
    println!(
        "Snapshot found: Snapshot_ID: {:?}, CreatedAt: {:?}",
        snapshot_id, start_time
    );

    // Get the current time for comparision.
    let now = Utc::now();
    let current_time = now.timestamp();
    //println!("Current Time is: {:?}", current_time);

    // For testing, lets consider snapshots older than 30 days to be deleted.
    //let thirty_days = 30 * 24 * 60 * 60; // TODO: Hardcoded for testing logic.
    let thirty_days = 24 * 60 * 60; // TODO: Hardcoded for testing logic.

    // Determine the timestamp, 30 days in the past.
    let last_thirty_days = current_time - thirty_days;

    let mut prepare_deletion = Vec::new();
    // Here, call we need a function to compare
    if start_time < last_thirty_days {
        prepare_deletion.push(snapshot_id);
    }
    prepare_deletion
}

pub async fn delete_snapshots(client: &Client, snapshot_ids: Vec<String>) -> Result<(), Error> {
    let snapshot_ids_to_delete = &snapshot_ids;
    println!(
        "The following snapshot(s) will be deleted because they are older than 1 day(s): {:?}",
        snapshot_ids_to_delete
    );

    for snapshot_id in snapshot_ids {
        match client
            .delete_snapshot()
            .snapshot_id(snapshot_id.clone())
            .dry_run(true)
            .send()
            .await
        {
            Ok(output) => {
                println!(
                    "Snapshot {:?} successfully deleted.\n
                    Output: {:?}",
                    snapshot_id, output
                );
            }
            Err(err) => {
                println!("Failed to delete snapshot {}:{}", snapshot_id, err);
            }
        }
    }
    Ok(())
}

//pub fn count_snapshots(client: &Client) {
//    let resp = client.describe_snapshots().owner_ids("self");
//    let snapshots = resp.snapshot_ids();
//    let length = snapshots
//
//    println!("Found {} snapshot(s)", length);
//}
