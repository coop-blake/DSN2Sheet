# CLI Data Utilities

## DSN2CSV

Windows utility to query a DSN then create a CSV text file from the results.

Currently under development.


# DSN2Sheet

#### Overview

Fetch data from an ODBC DSN using an SQL query and then upload the results to a specified Google Sheet using Google Sheets API. It takes command-line arguments for configuration.

#### Requirements

- Access to an ODBC data source with necessary permissions
- Google Service Account credentials JSON file (`googleCert.json` by default)

#### Usage

```
Usage: ./program <DSN> <sqlFile> <sheetID> <sheetRange> [googleCert]

Arguments:
- DSN: Data Source Name for the ODBC connection.
- sqlFile: Path to the SQL file containing the query.
- sheetID: ID of the Google Sheet where data will be uploaded.
- sheetRange: Range within the Google Sheet where data will be placed (e.g., "Sheet1!A1").
- googleCert (optional): Path to the Google Service Account credentials JSON file (default: "googleCert.json").
```

#### Example

```
$ ./program "ODBC_DSN" "query.sql" "sheetID123" "Sheet1!A1"
$ ./program "ODBC_DSN" "query.sql" "sheetID123" "Sheet1!A1" "path/to/custom/googleCert.json"
```

#### Notes

- Ensure the ODBC DSN is correctly configured and accessible.
- The Google Service Account JSON credentials should have sufficient permissions to modify the specified Google Sheet.

#### Authors

- This program was authored by Blake Glanville.

