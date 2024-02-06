//use aws_sdk_ec2::Error;
mod configs;
mod snapshot_utils;

#[tokio::main]
async fn main() {
    match snapshot_utils::fetch_existing_snapshots_as_array().await {
        Ok(snapshot_ids) => {
            for snapshot_id in snapshot_ids {
                println!("{}", snapshot_id);
            }
        }
        Err(e) => eprintln!("Error fetching Snapshot IDs: {}", e),
    }
}
