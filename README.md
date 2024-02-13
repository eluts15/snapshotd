# snapshotd

## TODO
- I still need figure out how to pass --dry-run=false. It `will` currently delete without verification.  
- Add mechanism to verify before deletion.  


## Usage

```

Delete snapshots in AWS if they are older than the specified number of days.

Delete AWS snapshots older than the specified number of days.

Usage: snapshotd --days <VALUE>

Options:
      --days <VALUE>  
  -h, --help          Print help
  -V, --version       Print version

```

```

# Delets snapshots older than the number of days specified

/snapshotd --days 1
Snapshot found: Snapshot_ID: "snap-0a4db19b54e140275", CreatedAt: 1707799656
Snapshot found: Snapshot_ID: "snap-02ee1dfe2efa1a54a", CreatedAt: 1707192484
Snapshot found: Snapshot_ID: "snap-09b40966263b9e763", CreatedAt: 1707177621
Snapshot found: Snapshot_ID: "snap-02a7ab62e907c299e", CreatedAt: 1707799642
The following snapshot(s) will be deleted because they are older specified number of days: ["snap-02ee1dfe2efa1a54a", "snap-09b40966263b9e763"]
Snapshot "snap-02ee1dfe2efa1a54a" successfully deleted.
Snapshot "snap-09b40966263b9e763" successfully deleted.

# No specified snapshots meet the deletion window

./snapshotd --days 30
Snapshot found: Snapshot_ID: "snap-0a4db19b54e140275", CreatedAt: 1707799656
Snapshot found: Snapshot_ID: "snap-088edc1e43a0b0a5a", CreatedAt: 1707799938
Snapshot found: Snapshot_ID: "snap-0b2bbcc18c57ea3a5", CreatedAt: 1707799911
Snapshot found: Snapshot_ID: "snap-0beda24665a9e7773", CreatedAt: 1707799891
The following snapshot(s) will be deleted because they are older specified number of days: []


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




