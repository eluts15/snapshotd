use aws_sdk_ec2::types::Snapshot;
use aws_sdk_ec2::{Client, Error};
use chrono::{TimeZone, Utc};

pub fn process_snapshot(_client: &Client, snapshot: Snapshot, days: i64) -> Vec<String> {
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

    let start_time_human_readable = match snapshot.start_time {
        Some(time) => {
            let start_time = Utc.timestamp(time.secs(), 0);
            start_time.to_string()
        }
        None => {
            eprintln!("Snapshot start time not found");
            return Vec::new();
        }
    };

    println!(
        "Snapshot found: Snapshot_ID: {:?}, CreatedAt: {:?} {:?}",
        snapshot_id, start_time, start_time_human_readable
    );

    // Get the current time for comparision.
    let now = Utc::now();
    let current_time = now.timestamp();

    let specified_days_in_seconds = days; // User input -- convert number of days to seconds.

    let delete_window = current_time - specified_days_in_seconds;
    let mut prepare_deletion = Vec::new();
    // Here, call we need a function to compare
    if start_time < delete_window {
        prepare_deletion.push(snapshot_id);
    }
    prepare_deletion
}

pub async fn delete_snapshots(client: &Client, snapshot_ids: Vec<String>) -> Result<(), Error> {
    let snapshot_ids_to_delete = &snapshot_ids;
    println!(
        "The following snapshot(s) will be deleted because they are older specified number of days: {:?}",
        snapshot_ids_to_delete
    );

    for snapshot_id in snapshot_ids {
        match client
            .delete_snapshot()
            .snapshot_id(snapshot_id.clone())
            .send()
            .await
        {
            Ok(_output) => {
                println!("Snapshot {:?} successfully deleted.", snapshot_id);
            }
            Err(_err) => {
                println!(
                    "Failed to delete snapshot {}, Permission Issue. Check IAM Permission Set.",
                    snapshot_id
                );
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
