# Ardan Labs Presentation: Introducing Rust into Your Company Ecosystem

## By Herbert Wolverson ( herbert.wolverson@ardanlabs.com )

![](https://github.com/thebracket/ArdanUltimateRustFoundations/blob/main/images/ardanlabs-logo.png)

Accompanying code for the presentation on YouTube. The slide deck accompanying the
presentation is [here](./SlideDeck.pdf).

## Service Oriented Architecture Examples

**Project** | **Description**
--- | ---
[Web Server in 16 Lines of Code](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/helloworld_webservice) | 16 lines of Rust, using Axum and Tokio to provide a simple web service that returns "Hello World" in plain text.
[Add JSON with 10 more Lines of Code](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/hellojson_webservice) | 10 more lines of Rust, using Axum, Tokio and Serde to provide a simple web service that returns "Hello World" in JSON.
[Web Service with SQLite and JSON](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/hellodb_webservice) | We're up to 35 lines of Rust, and 8 lines of SQL now. SQLX applies database migrations on startup, and provides compile-time validation of SQL queries. Each request queries the database, serializes to JSON, and returns the result.
[How fast is the webservice?](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/hellodb_timed_client) | Create a simple CLI tool that calls the web service we've created, and times responses.
[How fast is the serialization?](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/timed_json_serialize) | Time just the serialization to JSON
[A TCP Socket Server](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/tcp_server) | A simple TCP socket server that accepts connections, parses a command and returns a JSON result.
[TCP Socket Client](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/tcp_server_client) | A TCP client that connects to the TCP server, and times the responses.

## Integrating with Existing Services

**Project** | **Description**
--- | ---
[A Python Service](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/mypylib) | A simple Python service that exports a Python-friendly function. The included Python script imports the function and executes it.
[Rust from Go](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/rustgo) | A simple Go service that calls a Rust library. The included Go script imports the library and executes it.

## Migrating Legacy Code

[Rust Building C](https://github.com/thebracket/ArdanLabs_RustInYourEnterprise/tree/main/crust)

A C library wrapped in a Rust library, with unit tests calling the legacy code.

