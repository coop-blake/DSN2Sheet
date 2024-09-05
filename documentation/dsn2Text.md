## DSN2Text

Windows Command Line Utility  
_(Pre-Release v0.1.2)_  
Intel [32-bit](https://github.com/coop-blake/DSN2Sheet/releases/download/v0.1.2/DSN2Text-dev-i686.exe) [64-bit](https://github.com/coop-blake/DSN2Sheet/releases/download/v0.1.2/DSN2Text-dev-X86_64.exes)  
Arm [64-bit](https://github.com/coop-blake/DSN2Sheet/releases/download/v0.1.2/DSN2Text-dev-aarch64.exes)

#### Overview

Query a DSN and create a CSV text file from the results.

#### Requirements

- Access to an ODBC data source with necessary permissions
- Write access to the output file path

#### Usage

To run the program, use the following command:

```
./DSN2Text.exe <DSN> <sqlFile> <filename> [separator]

Arguments:
   - <DSN>: The Data Source Name for the ODBC connection.
   - <sqlFile>: The file containing the SQL query to be executed.
   - <filename>: The file where the data will be written.
   - separator (optional): The value seperator (Default: Tab).
```

#### Examples

Create a comma separated file from query

```
$ ./DSN2Text.exe ODBC_DSN query.sql output.csv ","
```

Create a tab separated file from query

```
$ ./DSN2Text.exe ODBC_DSN query.sql output.txt
```

#### Notes

- Ensure the ODBC DSN is correctly configured and accessible.

[DSN2Sheet](dsn2Sheet.html) - Push a DSN Query to a Google Sheet
