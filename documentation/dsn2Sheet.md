## DSN2Sheet

#### Overview

Query a DSN and place the results in a Google Sheet.

#### Requirements

- Access to an ODBC data source with necessary permissions
- Google Service Account credentials JSON file (_googleCert.json_ by default)

#### Usage

You can invoke DSN2Sheet be providing either a single target or a file containing multiple targets.

  
./DSN2Sheet.exe <DSN> <sqlFile> [<sheetID> <sheetRange> | targetsFile] [googleCert]


Arguments:
- DSN: Data Source Name for the ODBC connection.
- sqlFile: Path to the SQL file containing the query.
- sheetID: ID of the Google Sheet where data will be uploaded.
- sheetRange: Range within the Google Sheet where data will be placed (e.g., "Sheet1!A1").
- targetsFile: Path to a text file containing multiple sheet IDs and ranges. If specified, `sheetID` and `sheetRange` are not required.
- googleCert (optional): Path to the Google Service Account credentials JSON file (default: "googleCert.json").


#### Examples

**Single Target**

C:\DSN2Sheet> DSN2Sheet ODBC_DSN query.sql sheetID123 Sheet1!A1  
C:\DSN2Sheet> DSN2Sheet ODBC_DSN query.sql sheetID123 "Sheet one!A1" path/to/custom/googleCert.json


**Multiple Targets**


    C:\DSN2Sheet> DSN2Sheet ODBC_DSN query.sql targets.txt  
    C:\DSN2Sheet> DSN2Sheet.exe ODBC_DSN query.sql targets.txt path\to\custom\googleCert.json


Each line of your _targets.txt_ should contain a Sheet ID and range, separated by an "@" symbol.


**_example targets.txt_**

    18sm6sQSPceJjFijzj0qTBftQWkcHWRKE53pQ04iFl8c@'Test 2'!D11
    18sm6sQSPceJjFijzj0qTBftQWkcHWRKE53pQ04iFl8c@'Test 2'!D13
    18sm6sQSPceJjFijzj0qTBftQWkcHWRKE53pQ04iFl8c@'Test 2'!D15
    18sm6sQSPceJjFijzj0qTBftQWkcHWRKE53pQ04iFl8c@'Test 2'!D17
    18sm6sQSPceJjFijzj0qTBftQWkcHWRKE53pQ04iFl8c@'Test 2'!D19
    18sm6sQSPceJjFijzj0qTBftQWkcHWRKE53pQ04iFl8c@'Test 2'!D21

#### Notes

- Ensure the ODBC DSN is correctly configured and accessible.
- The Google Service Account JSON credentials should have sufficient permissions to modify the specified Google Sheets.

Unsigned binaries built by github actions can be found in [releases](https://github.com/coop-blake/DSN2Sheet/releases/tag/v0.1.2).  


[DSN2Text](dsn2Text.html) - Save DSN Query as text file