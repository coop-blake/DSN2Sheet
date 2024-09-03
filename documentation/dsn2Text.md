## DSN2Text

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

Unsigned binaries built by github actions can be found in [releases](https://github.com/coop-blake/DSN2Sheet/releases/tag/v0.1.2).  


[DSN2Sheet](dsn2Sheet.html) - Push a DSN Query to a Google Sheet  