mod snapshot_utils;
use crate::snapshot_utils::delete_snapshots;
use crate::snapshot_utils::process_snapshot;
use aws_sdk_ec2::Client;
use clap::{arg, Command};

#[tokio::main]
async fn main() {
    // AWS credential setup.
    let shared_config = aws_config::from_env()
        .credentials_provider(
            aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name("deployments-dev")
                .build(),
        )
        .load()
        .await;

    let client = Client::new(&shared_config);

    // Cli component.
    let matches = Command::new("snapshotd")
        .version("1.0")
        .about("Delete AWS snapshots older than the specified number of days.")
        .arg(arg!(--days <VALUE>).required(true))
        .get_matches();

    let days_specified = matches.get_one::<String>("days");
    let dt_string = days_specified.unwrap();
    let timestamp: i64 = dt_string.parse().unwrap();
    // Take the number of days the user provides, convert it to unix_timestamp
    let converted_dt = timestamp * 24 * 60 * 60; // 1 Day in seconds

    // Perform the request
    // TODO: Add Pagination.
    let response = client.describe_snapshots().owner_ids("self").send().await;
    if let Err(err) = response {
        eprintln!("Error describing snapshots: {:?}", err);
        return;
    }

    let mut snapshots_to_delete = Vec::new();

    // Process each snapshot
    if let Some(snapshots) = response.unwrap().snapshots {
        for snapshot in snapshots {
            // Call process_snapshot with snapshot and shared_config
            snapshots_to_delete.extend(process_snapshot(&client, snapshot, converted_dt));
        }
    }

    let snapshot_ids_to_delete = snapshots_to_delete;
    _ = delete_snapshots(&client, snapshot_ids_to_delete).await;
}
