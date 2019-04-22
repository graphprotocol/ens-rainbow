Convert ENS's rainbow table data into a CSV format we can ingest with Postgres' `COPY`.

Run this as `cat preimages-* | cargo run --release > data.csv`. The output is a CSV with quote ' and delimiter |

Takes a while (10 minutes) on my machine and results in a 30GB file (133M entities)

Could be made faster by matching regexps instead of deserializing JSON and probably by mmapping files instead of reading from stdin
