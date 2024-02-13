mod snapshot_utils;
use crate::snapshot_utils::delete_snapshots;
use crate::snapshot_utils::process_snapshot;
use aws_sdk_ec2::Client;

#[tokio::main]
async fn main() {
    let shared_config = aws_config::from_env()
        .credentials_provider(
            aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name("deployments-dev")
                .build(),
        )
        .load()
        .await;

    let client = Client::new(&shared_config);

    // Perform the request
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
            snapshots_to_delete.extend(process_snapshot(&client, snapshot));
        }
    }

    let snapshot_ids_to_delete = snapshots_to_delete;
    _ = delete_snapshots(&client, snapshot_ids_to_delete).await;
}
