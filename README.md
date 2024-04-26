# CS_317_Database_project

A my sql powered movie database project for CS 317. My first project in Rust and SQL.

Here are some basic instructions to get the project up and running on your local machine.

The exe is ready to go, but expects the database to be aviable on this URL:

`DATABASE_URL=mariadb://root:root@localhost:3306/movies`

To build the project, you will need to have Rust installed on your machine. You can install Rust by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install

Then it can be installed with: `cargo run --release`

The .env file contains the URL that the program will use to look for the database. You can change this to match how your database is set up.
