## Importing the rainbow tables

You will need an already initialized `graph-node` database. You will also
need to be able to run `psql` and connect to the `graph-node` database. If
your database is sharded, this data needs to be imported into the primary
database

#### Download the SQL dump file from Goolge cloud storage:
  * Compressed with `gzip` (5.9 GB):
    [ens_names.sql.gz](https://storage.cloud.google.com/ens-files/ens_names.sql.gz) ([sha256](https://storage.googleapis.com/ens-files/ens_names.sql.gz.sha256sum))
  * Compressed with `zstd` (5.6 GB):
    [ens_names.sql.zst](https://storage.googleapis.com/ens-files/ens_names.sql.zst) ([sha256](https://storage.googleapis.com/ens-files/ens_names.sql.zst.sha256sum))

#### Import the dump:

The `psql` command for the import has to be run as the same user as the
user that `graph-node` uses to connect to the database.

```
zcat ens_names.sql.gz | psql graph
```
or
```
zstdcat ens_names.sql.zst | psql graph
```

## Data preparation

These are the steps to generate the `ens_names.sql.gz` file, and are not
needed for just importing that data.

Convert ENS's rainbow table data into a SQL script that we can ingest with
`psql`, similar to a plain text dump from `pg_dump`.

Download input for rainbow tables via `gsutil cp gs://ens-files/preimages/* .`

Run this as `cat preimages-* | cargo run --release | gzip > data.sql`.

Takes a while (10 minutes) on my machine and results in a 6GB file (133M entities)

### Exporting the prepared data:
```
pg_dump -c -x -O --if-exists --no-tablespaces -t ens_names -f /var/tmp/ens_names.sql.gz -Z9
```
