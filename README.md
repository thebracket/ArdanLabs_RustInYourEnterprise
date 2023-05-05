# Ardan Labs Presentation: Introducing Rust into Your Company Ecosystem

## By Herbert Wolverson ( herbert.wolverson@ardanlabs.com )

Accompanying code for the presentation at (placeholder).

## Service Oriented Architecture Examples

**Project** | **Description**
--- | ---
[Web Server in 16 Lines of Code](./hello_webservice) | 16 lines of Rust, using Axum and Tokio to provide a simple web service that returns "Hello World" in plain text.
[Add JSON with 10 more Lines of Code](./hellojson_webservice/) | 10 more lines of Rust, using Axum, Tokio and Serde to provide a simple web service that returns "Hello World" in JSON.
[Web Service with SQLite and JSON](./hellodb_webservice/) | We're up to 35 lines of Rust, and 8 lines of SQL now. SQLX applies database migrations on startup, and provides compile-time validation of SQL queries. Each request queries the database, serializes to JSON, and returns the result.
[How fast is the webservice?](./hellodb_timed_client/) | Create a simple CLI tool that calls the web service we've created, and times responses.
[How fast is the serialization?](./timed_json_serialize/) | Time just the serialization to JSON
[A TCP Socket Server](./tcp_server/) | A simple TCP socket server that accepts connections, parses a command and returns a JSON result.
[TCP Socket Client](./tcp_client/) | A TCP client that connects to the TCP server, and times the responses.

## Integrating with Existing Services

**Project** | **Description**
--- | ---
[A Python Service](./mymath/) | A simple Python service that exports a Python-friendly function. The included Python script imports the function and executes it.

