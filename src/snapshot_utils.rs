use aws_sdk_ec2::types::Snapshot;
use aws_sdk_ec2::Client;
use std::time::Instant;

pub fn process_snapshot(_client: &Client, snapshot: Snapshot) {
    let now = Instant::now();
    print!("Current Time is: {:?}", now);
    // Extract snapshot ID
    let snapshot_id = match snapshot.snapshot_id {
        Some(id) => id,
        None => {
            eprintln!("Snapshot ID not found");
            return;
        }
    };

    // Extract snapshot start time
    let start_time = match snapshot.start_time {
        Some(time) => time.as_secs_f64(),
        None => {
            eprintln!("Snapshot start time not found");
            return;
        }
    };

    println!("ID: {:?}, CreatedAt: {:?}", snapshot_id, start_time);
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
