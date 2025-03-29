# Rust URL Shortener Service

A full‑stack URL shortening service built entirely in **Rust** using Actix‑web and Diesel with SQLite. This service allows users to:

- **Create Short URLs:** Submit a long URL and receive a unique, shortened URL.
- **List URLs:** Retrieve a list of all shortened URLs.
- **Redirection:** Visit a short URL to be automatically redirected to the original URL.

---

## Why Use Rust?

- **Performance:** Rust is known for its high performance and low overhead, making it ideal for building fast, scalable web services.
- **Memory Safety:** Rust’s ownership system guarantees memory safety without needing a garbage collector, which minimizes runtime errors.
- **Concurrency:** Rust provides powerful concurrency primitives that help in building robust, multi-threaded applications.
- **Modern Tooling:** With a growing ecosystem, Rust offers excellent tooling (cargo, rustfmt, clippy) for building and maintaining reliable software.
- **Reliability:** Rust’s strict compile-time checks catch many bugs early in development, ensuring a more reliable codebase.

---

## Technology Stack

- **Web Framework:** [Actix‑web](https://actix.rs/)
- **ORM & Database:** [Diesel](https://diesel.rs/) with SQLite
- **Environment Management:** dotenvy for environment variables
- **Utilities:** rand for generating short codes, chrono for timestamps

---

## File Structure

```
rust-url-shortener/
├── Cargo.toml
├── .env
├── migrations/
│   └── 20230310123456_create_urls/
│       ├── up.sql
│       └── down.sql
├── src/
│   ├── main.rs
│   ├── db.rs
│   ├── models.rs
│   ├── schema.rs
│   └── routes.rs
└── README.md
```

---

## Getting Started

### Prerequisites

- **Rust & Cargo:** Install via [rustup.rs](https://rustup.rs/)
- **SQLite:** Ensure SQLite is installed on your system.
- **Diesel CLI:** Install Diesel CLI with SQLite support by running:
  ```bash
  cargo install diesel_cli --no-default-features --features sqlite
  ```

### Setup

1. **Clone the Repository or Create the Project Folder**

   You can create the file structure using the following terminal commands:

   ```bash
   # Create project directory and initialize a new binary project
   mkdir rust-url-shortener
   cd rust-url-shortener
   cargo init --bin

   # Create directories for Diesel migrations
   mkdir -p migrations/20230310123456_create_urls

   # Create migration SQL files
   touch migrations/20230310123456_create_urls/up.sql
   touch migrations/20230310123456_create_urls/down.sql

   # Create additional source files
   cd src
   touch db.rs models.rs schema.rs routes.rs
   cd ..
   ```

2. **Create a `.env` File**

   In the project root, create a file named `.env` with the following content (adjust values as needed):

   ```env
   DATABASE_URL=rust_url_shortener.db
   BASE_URL=http://localhost:8080
   ```

3. **Run Diesel Migrations**

   Set up the database schema by running:
   ```bash
   diesel migration run
   ```

---

## Running the Application

1. **Build and Run the Server**

   ```bash
   cargo run
   ```

   The server will start at [http://localhost:8080](http://localhost:8080).

2. **API Endpoints**

  - **Create URL:**  
    `POST /` with JSON body:
    ```json
    {
      "original_url": "https://example.com"
    }
    ```
    Returns the created URL entry with a generated short code.

  - **List URLs:**  
    `GET /` returns a list of shortened URLs.

  - **Redirection:**  
    `GET /{short_code}` redirects to the original URL if found.

---

## Customization

- **Changing the BASE_URL:**  
  Update the `BASE_URL` variable in your `.env` file to reflect your domain (e.g., `https://yourdomain.com`).

- **Extending Functionality:**  
  Modify or add additional endpoints in `src/routes.rs` and update models in `src/models.rs` as needed.

- **Integrating a Frontend:**  
  This example provides a robust backend API. You can build a frontend in your preferred technology to consume this API.

---

## Contributing

Feel free to fork this repository and submit pull requests with improvements or customizations. For issues or feature requests, please open an issue.

---

## License

This project is licensed under the MIT License.

---

## Authors

The UNC-CH Google Developer Student Club (GDSC) team.
