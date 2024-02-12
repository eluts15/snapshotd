mod snapshot_utils;
use crate::snapshot_utils::process_snapshot;
use aws_sdk_ec2::Client;
//use aws_smithy_types::date_time::DateTime;

// Fucking retarted
//fn parse_rfc3339_to_datetime(datetime_str: &str) -> DateTime {
//    // Parse the RFC 3339 formatted string into the components
//    let datetime_parts: Vec<&str> = datetime_str.split('T').collect();
//    let date_parts: Vec<&str> = datetime_parts[0].split('-').collect();
//    let time_parts: Vec<&str> = datetime_parts[1].split(':').collect();
//
//    //Extract the individual components
//    let year: u64 = date_parts[0].parse().expect("Invalid year");
//    let month: u64 = date_parts[1].parse().expect("Invalid month");
//    let day: u64 = date_parts[2]
//        .split('T')
//        .next()
//        .unwrap()
//        .parse()
//        .expect("Invalid day");
//    let hour: u64 = time_parts[0].parse().expect("Invalid hour");
//    let minute: u64 = time_parts[1].parse().expect("Invalid minute");
//    let second: u64 = time_parts[2]
//        .split('.')
//        .next()
//        .unwrap()
//        .parse()
//        .expect("Invalid second");
//
//    // Create the DateTime object
//    DateTime::from_secs(second.try_into().unwrap())
//}

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

    let x = snapshots_to_delete;
    println!("To Delete:{:?}", x);
}
