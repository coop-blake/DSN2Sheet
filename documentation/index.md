# Windows Command Line Utilities
A set of command line utilities for moving data from an ODBC Data source to either a remote Google sheet or local text file.

Unsigned binaries built by github actions can be found in [release](https://github.com/coop-blake/DSN2Sheet/releases).  

You can also build your own from the [repository](https://github.com/coop-blake/DSN2Sheet) after [installing rust](https://www.rust-lang.org/tools/install) with the following command.

```
cargo build --release
```

This will create the two programs outlined below

[DSN2Sheet](dsn2Sheet.md) - Push a DSN Query to a Google Sheet  
[DSN2Text](dsn2Text.md) - Save DSN Query as text file
