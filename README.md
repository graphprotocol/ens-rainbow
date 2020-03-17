Convert ENS's rainbow table data into a SQL script that we can ingest with
`psql`, similar to a plain text dump from `pg_dump`.

Download input for rainbow tables via `gsutil cp gs://ens-files/* .`

Run this as `cat preimages-* | cargo run --release | gzip > data.sql`.

Takes a while (10 minutes) on my machine and results in a 6GB file (133M entities)

## Data import and export

### Exporting the prepared data from staging:
```
pg_dump -c -O --no-tablespaces -t ens_names -f /var/tmp/ens_names.sql.gz -Z9
```

The data is available in [Google cloud
storage](https://storage.cloud.google.com/ens-files/ens_names.sql.gz) There
is also a manually generated
[addendum](https://storage.cloud.google.com/ens-files/ens_names_2.csv.gz)

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
2a. Import the addendum:
```
zcat ens_names_2.csv.gz | psql -c 'copy ens_names(name, hash) from stdin with (format csv)'
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
