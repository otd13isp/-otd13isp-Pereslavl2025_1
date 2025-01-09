# Data selection and implementation of an aggregated query of sensitive data using DuckDB database and Rust language
## Free Software (free software)

The application is free software.

## Functions
- Loading CSV, JSON, Parquet files into DuckDB databases
- Connecting DuckDB databases to MySQL and Postgres databases to use a table of these databases in an SQL query
- Searching and aggregating information in DuckDB databases
- Application of regular expressions to search by pattern
- Lemmatization and sorting of text in a text file and search string
- Converting letters in a text file and search string to lower case
- Create independent threads to search for matches in a DuckDB file
- Presentation of information in the browser

## Requirements:
- [DuckDB](https://duckdb.org) - for data accumulation and processing
- [Rust](https://www.rust-lang.org) - to create applications 
- [Actix Web](https://actix.rs) -  Web framework for Rust
- [JavaScript](https://www.ecma-international.org/publications-and-standards/standards/ecma-262) - to send the search string to the server
- [C++](https://gcc.gnu.org) - for compiling packages Rust and DuckDB
- [Python](https://www.python.org) - for lemmatization and text sorting
 
 The application is also open source with code on Github

## Installation

Installation on Linux (Fedora) is performed with administrator rights.

Run the commands to install openssl:

\# dnf install openssl

\# dnf install pkg-config perl-FindBin openssl-devel


To install C++ for compiling packages Rust and DuckDB:

\# dnf install gcc-c++

To run the project you need to install Rust.

Installing Rust:

\# dnf install rust cargo

To install dependencies correctly, you need to edit the Cargo.toml file:

[package]

name = "bookrust2"

version = "0.1.0"

edition = "2021"

[dependencies]

duckdb = { version = "1.1.1", features = ["bundled"] }

libduckdb-sys = "1.1.1"

actix-web = "4.9.0"

serde = { version = "1.0.217", features = ["derive"] }

regex = "1.11.1"

Installing DuckDB:

To install DuckDB on Fedora, simply copy the duckdb executable file to the **/usr/bin** directory

Installing Python:

\# dnf install python

Installation pip:

\# dnf install python3-pip

Download text lemmatization package:

\# pip install pymystem3

The Rust program text is contained in the **src/main.rs** file.

You need to specify the path to the DuckDB database in the **src/main.rs** file.

Create a **public** subdirectory in the project directory. Create a **html** subdirectory. The **html** subdirectory contains the **index.html** files and file DuckDB data base **searchdb.duckdb**.  File  **searchdb.duckdb**  will be used for searching

## Usage
To start the project server, run the command

\$ cargo run

in the project directory

Type http://localhost:8080 to request to the server. Enter the search data in the form of sorted lemmas in the search line.

## License
MIT

