mod snapshot_utils;
use crate::snapshot_utils::fetch_existing_snapshots_as_array_test;

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

    fetch_existing_snapshots_as_array_test(&client).await;
}
