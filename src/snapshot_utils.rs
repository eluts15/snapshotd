use aws_sdk_ec2::types::Snapshot;
use aws_sdk_ec2::Client;
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
    println!("ID: {:?}, CreatedAt: {:?}", snapshot_id, start_time);

    // Get the current time for comparision.
    let now = Utc::now();
    let current_time = now.timestamp();
    //println!("Current Time is: {:?}", current_time);

    // For testing, lets consider snapshots older than 30 days to be deleted.
    let thirty_days = 30 * 24 * 60 * 60; // TODO: Hardcoded for testing logic.

    // Determine the timestamp, 30 days in the past.
    let last_thirty_days = current_time - thirty_days;

    let mut prepare_deletion = Vec::new();
    // Here, call we need a function to compare
    if start_time < last_thirty_days {
        println!("The following snapshot will be deleted: {}", snapshot_id);
        prepare_deletion.push(snapshot_id);
    } else {
        println!("No snapshots found matching the specified parameters.");
    }

    prepare_deletion
}

pub fn delete_snapshot(client: &Client, snapshot_ids: Vec<String>) {
    //let response = client.delete_snapshot().;
    //let delete_these = &snapshot_ids;
}

//pub async fn count_snapshots() -> Result<(), Error> {k
//    let config = match load_config().await {
//        Ok(config) => config,
//        Err(e) => {
//            eprintln!("Error loading AWS SDK configuration: {:?}", e);
//            return Err(e); // Convert the error type if necessary
//        }
//    };
//
//    let ec2 = Client::new(&config);
//
//    let resp = ec2.describe_snapshots().owner_ids("self").send().await?;
//    let snapshots = resp.snapshots();
//    let length = snapshots.len();
//
//    println!("Found {} snapshot(s)", length);
//    Ok(())
//}
