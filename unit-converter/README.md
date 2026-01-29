# Unit Converter

A simple web-based unit conversion application with a Rust backend and a minimal frontend.

The application supports converting values between different units of **length**, **weight**, and **temperature** using an HTTP API.

---

## Project Structure

```
.
├── index.html
└── unit-converter-backend
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── main.rs
    └── static
        ├── scripts
        │   └── script.js
        └── styles
            └── style.css
```

---

## Technologies

### Backend

* Rust
* Axum
* Tokio
* Serde
* Anyhow
* Tower HTTP

### Frontend

* HTML
* CSS
* JavaScript (Fetch API)

---

## Supported Conversions

### Length

* millimeter
* centimeter
* meter
* kilometer
* inch
* foot
* yard
* mile

### Weight

* milligram
* gram
* kilogram
* pound
* ounce

### Temperature

* Celsius
* Fahrenheit
* Kelvin

---

## Running the Application

### Build and run the backend

```bash
cargo run
```

The server will start on:

```
127.0.0.1:3002
```

### Open in browser

Open the following address in your browser:

```
http://127.0.0.1:3002
```

---

## API

### POST `/api/convert`

#### Request body

```json
{
  "value": 10,
  "from": "meter",
  "to": "kilometer"
}
```

#### Successful response

```json
{
  "value": 0.01
}
```

#### Error response

```json
{
  "error": "invalid query"
}
```

---

## Notes

* Static frontend files are served by the backend.
* The application does not require a database.
* All requests and responses use JSON.

---

## License

This project is provided for educational and experimental purposes.
