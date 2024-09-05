# Windows Command Line Utilities

Links to binaries compiled by github actions can be found on the [documentation page](https://coop-blake.github.io/DSN2Sheet/)

You can also build your own from this repository after [installing rust](https://www.rust-lang.org/tools/install)

```
cargo build --release
```

This will create the two programs outlined below

## DSN2Sheet

#### Overview

Query a DSN and place the results in a Google Sheet.

#### Requirements

- Access to an ODBC data source with necessary permissions
- Google Service Account credentials JSON file (`googleCert.json` by default)

#### Example Commands

```
$ ./DSN2Sheet.exe ODBC_DSN query.sql sheetID123 Sheet1!A1
$ ./DSN2Sheet.exe ODBC_DSN query.sql sheetID123 "Sheet one!A1" path/to/custom/googleCert.json
```

## DSN2Text

Query a DSN and create a CSV text file from the results.

#### Requirements

- Access to an ODBC data source with necessary permissions
- Write access to the output file path

#### Example Commands

Create a comma separated file from query

```
$ ./DSN2Text.exe ODBC_DSN query.sql output.csv ","
```

Create a tab separated file from query

```
$ ./DSN2Text.exe ODBC_DSN query.sql output.txt
```
