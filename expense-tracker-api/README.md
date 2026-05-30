# 💸 Expense Tracker API

A modern, secure, and production-ready RESTful API for an Expense Tracker application, built using **Go (Golang)**, **Fiber v3**, and a pure-Go **SQLite** driver.

This is the final beginner milestone project in the [Roadmap.sh Backend Developer](https://roadmap.sh/backend) learning path. It features complete JWT-based stateless authentication, strict ownership-based authorization to prevent IDOR (Insecure Direct Object Reference) vulnerabilities, and a robust query-building system for advanced filtering.

---

## 🛠️ Tech Stack & Key Libraries

- **Language:** Go 1.26.3
- **Web Framework:** [Fiber v3](https://github.com/gofiber/fiber) (Express-like, high-performance router)
- **Database:** [SQLite](https://sqlite.org) (Serverless, stored locally in `expenses.db`)
- **SQL Driver:** `modernc.org/sqlite` (Pure Go SQLite driver, no CGO required!)
- **Security:** `golang.org/x/crypto/bcrypt` (Secure password hashing)
- **Token Auth:** [JWT v5](https://github.com/golang-jwt/jwt) (Stateless authentication & session management)

---

## 🌟 Key Features

- [x] **User Authentication:** Fully secure registration and login endpoints utilizing bcrypt for passwords and signed JWTs.
- [x] **Stateless Security Middleware:** Automatic request validation, extraction of user identifiers from the `Authorization: Bearer <token>` header, and context injection.
- [x] **Secure CRUD Operations:** Users can create, read, update, and delete expenses only from their own separate workspaces.
- [x] **Database-Level Ownership Guards:** `DELETE` and `UPDATE` operations are fully verified at the SQL level (`WHERE id = ? AND user_id = ?`) to secure the application against database tampering and IDOR vulnerabilities.
- [x] **Advanced Expense Filtering:** Filter past expenses using dynamic queries:
  - **Preset Periods:** `past_week` (default), `past_month`, `past_3_months`.
  - **Custom Ranges:** Filter with exact `start_date` and `end_date` boundary queries.
  - **Category Filter:** Easily filter by specific preset categories.

---

## 📦 Project Structure

```text
├── db.go            # SQLite initialization, migrations, and CRUD operations
├── db_test.go       # Thorough automated unit test suite (100% database coverage)
├── go.mod           # Go modules file and dependencies
├── handlers.go      # REST endpoint handlers (Request parsing & response building)
├── jwt.go           # Signed JWT token generation and parsing utilities
├── justfile         # Handy project task automation
├── main.go          # Application entry point
├── middleware.go    # HTTP Authorization middleware
├── models.go        # Shared application data models, DTOs, and constants
└── utils.go         # Common utilities (password hashing and validation)
```

---

## 📊 Expense Categories

The API enforces validation for the following predefined expense categories:
- 🛒 `groceries`
- 🎮 `leisure`
- 🔌 `electronics`
- 💡 `utilities`
- 👕 `clothing`
- 🏥 `health`
- 🌀 `others`

---

## 🚀 API Documentation

### Authentication Endpoints

#### 1. Register User
- **Method:** `POST`
- **Path:** `/api/auth/register`
- **Request Body:**
  ```json
  {
    "name": "Alex",
    "email": "alex@example.com",
    "password": "securepassword"
  }
  ```
- **Response (201 Created):** Returns a signed JWT token string.

#### 2. User Login
- **Method:** `POST`
- **Path:** `/api/auth/login`
- **Request Body:**
  ```json
  {
    "email": "alex@example.com",
    "password": "securepassword"
  }
  ```
- **Response (200 OK):** Returns a signed JWT token string.

---

### Expense Endpoints (All require `Authorization: Bearer <token>`)

#### 3. List & Filter Expenses
- **Method:** `POST` (Mapped to `GetExpensesHandler`)
- **Path:** `/api/expenses/list`
- **Request Body:**
  ```json
  {
    "period": "past_month",
    "category": "groceries"
  }
  ```
  *Or for custom date filtering:*
  ```json
  {
    "period": "custom",
    "start_date": "2026-05-01",
    "end_date": "2026-05-15"
  }
  ```
- **Response (200 OK):** Array of `DBExpense` objects matching filters.

#### 4. Add Expense
- **Method:** `POST`
- **Path:** `/api/expenses`
- **Request Body:**
  ```json
  {
    "amount": 42.50,
    "category": "groceries",
    "description": "Weekly grocery run",
    "date": "2026-05-28"
  }
  ```
- **Response (201 Created):** Full `DBExpense` object.

#### 5. Update Expense
- **Method:** `PUT`
- **Path:** `/api/expenses`
- **Request Body:**
  ```json
  {
    "id": 1,
    "amount": 45.00,
    "description": "Weekly grocery run (updated pricing)"
  }
  ```
- **Response (200 OK):** Updated `DBExpense` object.

#### 6. Delete Expense
- **Method:** `DELETE`
- **Path:** `/api/expenses/:id`
- **Response (200 OK):** Deleted `DBExpense` object.

---

## 🧪 Testing

The codebase comes equipped with an extensive automated unit testing suite located in `db_test.go`. The tests isolate database logic using SQLite in-memory databases (`:memory:`) to guarantee speed, consistency, and clean test cleanup.

To execute all tests with high verbosity, run:

```bash
just test
# or
go test -v ./...
```
