# snapshotd


## Usage

```

Delete snapshots in AWS if they are older than the specified number of days.

Delete AWS snapshots older than the specified number of days.
Running without --write will make it run in dry-run mode.

Usage: snapshotd --days <VALUE> --write <VALUE>

Options:
      --days <VALUE>  
  -h, --help          Print help
  -V, --version       Print version

Both, --days, --write are required fields.

```

# Delets snapshots older than the number of days specified

```
### dry-run mode

./snapshotd --days 1 --write=false
Snapshot found: Snapshot_ID: "snap-0a4db19b54e140275", CreatedAt: 1707799656 "2024-02-13 04:47:36 UTC"
Snapshot found: Snapshot_ID: "snap-088edc1e43a0b0a5a", CreatedAt: 1707799938 "2024-02-13 04:52:18 UTC"
Snapshot found: Snapshot_ID: "snap-0b2bbcc18c57ea3a5", CreatedAt: 1707799911 "2024-02-13 04:51:51 UTC"
Snapshot found: Snapshot_ID: "snap-0beda24665a9e7773", CreatedAt: 1707799891 "2024-02-13 04:51:31 UTC"
The following snapshot(s) will be deleted because they are older specified number of days: ["snap-0a4db19b54e140275", "snap-088edc1e43a0b0a5a", "snap-0b2bbcc18c57ea3a5", "snap-0beda24665a9e7773"]
Failed to delete snapshot snap-0a4db19b54e140275, Permission Issue. Check IAM Permission Set.
Failed to delete snapshot snap-088edc1e43a0b0a5a, Permission Issue. Check IAM Permission Set.
Failed to delete snapshot snap-0b2bbcc18c57ea3a5, Permission Issue. Check IAM Permission Set.
Failed to delete snapshot snap-0beda24665a9e7773, Permission Issue. Check IAM Permission Set.


### success deletion
./snapshotd --days 1 --write=true
Snapshot found: Snapshot_ID: "snap-0a4db19b54e140275", CreatedAt: 1707799656 "2024-02-13 04:47:36 UTC"
Snapshot found: Snapshot_ID: "snap-088edc1e43a0b0a5a", CreatedAt: 1707799938 "2024-02-13 04:52:18 UTC"
Snapshot found: Snapshot_ID: "snap-0b2bbcc18c57ea3a5", CreatedAt: 1707799911 "2024-02-13 04:51:51 UTC"
Snapshot found: Snapshot_ID: "snap-0beda24665a9e7773", CreatedAt: 1707799891 "2024-02-13 04:51:31 UTC"
The following snapshot(s) will be deleted because they are older specified number of days: ["snap-0a4db19b54e140275", "snap-088edc1e43a0b0a5a", "snap-0b2bbcc18c57ea3a5", "snap-0beda24665a9e7773"]
Snapshot "snap-0a4db19b54e140275" successfully deleted.
Snapshot "snap-088edc1e43a0b0a5a" successfully deleted.
Snapshot "snap-0b2bbcc18c57ea3a5" successfully deleted.
Snapshot "snap-0beda24665a9e7773" successfully deleted.

```

## Setup

Create an IAM User with the minumum permission set.

```

{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "VisualEditor0",
      "Effect": "Allow",
      "Action": [
	"ec2:DeleteSnapshot",
	"ec2:DescribeSnapshots"
      ],
      "Resource": "*"
    }
  ]
}

```
Create an IAM user and attach the permission set (~/.aws/.credentials)

```

[your-profile]
aws_access_key_id=YOUR_ACCCES_KEY_ID
aws_secret_access_key=YOUR_SECRET_ACCESS_KEY
region=YOUR_REGION

```

Currently uses .env to map the PROFILE to credentials found in ~/.aws/credentials (not ideal, redundant)  
Create a `.env` file in the root directory.  

```
PROFILE=YOUR_PROFILE
```




