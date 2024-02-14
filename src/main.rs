mod snapshot_utils;
use crate::snapshot_utils::delete_snapshots;
use crate::snapshot_utils::process_snapshot;
use aws_sdk_ec2::Client;
use clap::{value_parser, Arg, Command};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // AWS credential setup.
    dotenv().ok();

    let env_profile = "PROFILE";

    let profile_name = dotenv::var(env_profile).unwrap();
    let shared_config = aws_config::from_env()
        .credentials_provider(
            aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name(profile_name)
                .build(),
        )
        .load()
        .await;

    let client = Client::new(&shared_config);

    // Cli component.
    let cmd = Command::new("snapshotd")
        .version("1.0")
        .about("Delete AWS snapshots older than the specified number of days.")
        .arg(
            Arg::new("days")
                .short('d')
                .long("days")
                .required(true)
                .num_args(1)
                .help("Required -- specifies the number of days.  --days <1>"),
        )
        // still isnt obvious if i can pass in a second argument.
        .arg(
            Arg::new("write")
                .long("write")
                .value_name("BOOL")
                .value_parser(value_parser!(bool))
                .required(true)
                .action(clap::ArgAction::Set)
                .num_args(0..=1)
                .require_equals(true) // --write=<true/false>
                .default_missing_value("false")
                .help("Required -- specifies if the program will execute in dry_run mode or not. --write <true/false>"),
        )
        .get_matches();

    // parse the --days argument.
    let days = cmd
        .get_one::<String>("days")
        .expect("`days <int>` is required.")
        .to_string();
    let days_specified = days;
    let dt_string = days_specified;
    let timestamp: i64 = dt_string.parse().unwrap();
    // Take the number of days the user provides, convert it to unix_timestamp
    let converted_dt = timestamp * 24 * 60 * 60; // 1 Day in seconds

    // parse the --write argument.
    let write = cmd
        .get_one::<bool>("write")
        .expect("`bool --write <true/false>` required.");

    //let test_matchs = cmd.try_get_many::<bool>("--write");

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
    //_ = delete_snapshots(&client, snapshot_ids_to_delete, mode).await;
    _ = delete_snapshots(&client, snapshot_ids_to_delete, *write).await;
}
