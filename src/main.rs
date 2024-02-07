mod configs;
mod snapshot_utils;

#[tokio::main]
async fn main() {
    match snapshot_utils::prepare_snapshots_for_deletion_as_array().await {
        Ok(snapshots_info) => {
            println!("Snapshots Info:");
            for (snapshot_id, start_time, start_time_str) in snapshots_info {
                println!(
                    "- Snapshot ID: {}, Start Time: {} ({})",
                    snapshot_id, start_time, start_time_str
                );
            }
        }
        Err(e) => eprintln!("Error fetching snapshots info: {:?}", e),
    }

    match snapshot_utils::fetch_existing_snapshots_timestamps().await {
        Ok(snapshots_info) => {
            println!("Snapshots Info:");
            for start_time in snapshots_info {
                println!("Created At: {}", start_time);
            }
        }
        Err(e) => eprintln!("Error fetching snapshots info: {:?}", e),
    }
}
