use aws_config::{profile::ProfileFileCredentialsProvider, SdkConfig};
use aws_sdk_ec2::Error;
use dotenv::dotenv;

pub async fn load_config() -> Result<SdkConfig, Error> {
    dotenv().ok();

    let env_profile = "PROFILE";

    let profile_name = dotenv::var(env_profile).unwrap();

    let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(profile_name)
        .build();

    let config = aws_config::from_env()
        .credentials_provider(credentials_provider)
        .load()
        .await;

    Ok(config)
}
