## Importing the rainbow tables

You will need an already initialized `graph-node` database. You will also
need to be able to run `psql` and connect to the `graph-node` database. If
your database is sharded, this data needs to be imported into the primary
database

1. Download two data files from Goolge cloud storage:
[ens_names.sql.gz](https://storage.cloud.google.com/ens-files/ens_names.sql.gz)
and
[ens_names_2.csv.gz](https://storage.cloud.google.com/ens-files/ens_names_2.csv.gz)

2. Import the dump:
```
zcat ens_names.sql.gz | psql graph
```
2a. Import the addendum:
```
zcat ens_names_2.csv.gz | psql -c 'copy ens_names(name, hash) from stdin
with (format csv)
```

## Data preparation

These are the steps to generate the `ens_names.sql.gz` file, and are not
needed for just importing that data.

Convert ENS's rainbow table data into a SQL script that we can ingest with
`psql`, similar to a plain text dump from `pg_dump`.

Download input for rainbow tables via `gsutil cp gs://ens-files/* .`

Run this as `cat preimages-* | cargo run --release | gzip > data.sql`.

Takes a while (10 minutes) on my machine and results in a 6GB file (133M entities)

### Exporting the prepared data:
```
pg_dump -c -O --no-tablespaces -t ens_names -f /var/tmp/ens_names.sql.gz -Z9
```
