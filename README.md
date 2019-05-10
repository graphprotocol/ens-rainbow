Convert ENS's rainbow table data into a CSV format we can ingest with Postgres' `COPY`.

Download input for rainbow tables via `gsutil cp gs://ens-files/* .`

Run this as `cat preimages-* | cargo run --release > data.csv`. The output is a CSV with quote `'` and delimiter `|`

Takes a while (10 minutes) on my machine and results in a 30GB file (133M entities)

Could be made faster by matching regexps instead of deserializing JSON and probably by mmapping files instead of reading from stdin

**IMPORTANT:** we need to better document the transformation that was made
when importing this into staging.

## Data import and export

### Exporting the prepared data from staging:
```
pg_dump -c -O --no-tablespaces -t ens_names -f /var/tmp/ens_names.sql.gz -Z9
```

The data is available in [Google cloud
storage](https://storage.cloud.google.com/subgraph-dumps/ens_names.sql.gz)

### Importing it into production:
1. Make sure the `ens_names` table exists (in psql):
```
create table if not exists public.ens_names(
  hash varchar primary key,
  name varchar not null
);
```
2. Import the dump:
```
zcat ens_names.sql.gz | psql graph
```
3. Create a foreign key index on `data->'parent'->>'data'`:
```
create index domain_parent on sgdX.entities((data->'parent'->>'data'))
       where entity = 'Domain';
```

4. Update the names of existing entities:
```
update sgdX.entities
   set data = data || (select format('{"labelName": {"data": "%s",
                                                     "type": "String" } }',
                                     name)
                         from ens_names where hash = data->'labelHash'->>'data')
 where entity = 'Domain'
   and data->'labelName'->>'data' is null;
```
