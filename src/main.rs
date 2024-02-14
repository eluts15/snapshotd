mod cli;
mod snapshot_utils;
//use crate::cli::CmdArgs;
use crate::snapshot_utils::delete_snapshots;
use crate::snapshot_utils::process_snapshot;
use aws_sdk_ec2::Client;
use clap::{Arg, Command};

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
    let cmd = Command::new("snapshotd")
        .version("1.0")
        .about("Delete AWS snapshots older than the specified number of days.")
        .arg(
            Arg::new("days")
                .short('d')
                .long("days")
                .required(true)
                .help("specify the number of days."),
        )
        .arg(
            Arg::new("write")
                .long("write")
                //.required(false)
                .action(clap::ArgAction::SetFalse)
                .num_args(0)
                .help("omitting this defaults to 'dry-run' mode."),
        )
        // .arg(
        //     Arg::new("help")
        //         .short('?')
        //         .long("help")
        //         .action(clap::ArgAction::HelpShort),
        // )
        // .arg(
        //     Arg::new("version")
        //         .short('V')
        //         .long("version")
        //         .action(clap::ArgAction::Version),
        // )
        .get_matches();

    // Help
    //let err = cmd.clone().try_get_many(["snapshotd", "-?"]).unwrap_err();
    //assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);

    //// Version
    //let err = cmd.clone().try_get_many(["version"]).unwrap_err();
    //assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);

    //try_get_matches_from(vec!["snapshotd", "--days", "--write"]);
    //.try_get_matches_from(vec!["snapshotd", "--days", "--write"])

    //write: matches.get_flag("write"),

    let days = cmd
        .get_one::<String>("days")
        .expect("`days` is required.")
        .to_string();
    //let write: bool = cmd.get_one("write").is_some_and(true | false);

    //assert!(matches.contains_id("days"));
    //assert_eq!(matches.get_flag("days"), true);

    //assert!(matches.contains_id("write"));
    //assert_eq!(matches.get_flag("write"), false | true);

    // these appear to be working correctly.
    let days_specified = days;
    let dt_string = days_specified;
    let timestamp: i64 = dt_string.parse().unwrap();
    // Take the number of days the user provides, convert it to unix_timestamp
    let converted_dt = timestamp * 24 * 60 * 60; // 1 Day in seconds
    println!("Timestamp: {:?}", converted_dt);

    // passed to fn delete_snapshots
    let write = true;

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
    _ = delete_snapshots(&client, snapshot_ids_to_delete, write).await;
}
