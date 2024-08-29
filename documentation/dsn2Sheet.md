## DSN2Sheet

#### Overview

Query a DSN and place the results in a Google Sheet.

#### Requirements

- Access to an ODBC data source with necessary permissions
- Google Service Account credentials JSON file (`googleCert.json` by default)

#### Usage

```
./DSN2Sheet.exe <DSN> <sqlFile> <sheetID> <sheetRange> [googleCert]

Arguments:
- DSN: Data Source Name for the ODBC connection.
- sqlFile: Path to the SQL file containing the query.
- sheetID: ID of the Google Sheet where data will be uploaded.
- sheetRange: Range within the Google Sheet where data will be placed (e.g., "Sheet1!A1").
- googleCert (optional): Path to the Google Service Account credentials JSON file (default: "googleCert.json").
```

#### Examples

```
$ ./DSN2Sheet.exe ODBC_DSN query.sql sheetID123 Sheet1!A1
$ ./DSN2Sheet.exe ODBC_DSN query.sql sheetID123 "Sheet one!A1" path/to/custom/googleCert.json
```

#### Notes

- Ensure the ODBC DSN is correctly configured and accessible.
- The Google Service Account JSON credentials should have sufficient permissions to modify the specified Google Sheet.
