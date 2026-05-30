package main

import (
	"context"
	"database/sql"
	"errors"
	"time"

	_ "modernc.org/sqlite"
)

type DB struct {
	*sql.DB
}

func NewDB() (*DB, error) {
	db, err := sql.Open("sqlite", "./expenses.db")
	if err != nil {
		return nil, err
	}

	if err := db.Ping(); err != nil {
		return nil, err
	}

	if _, err := db.Exec("PRAGMA foreign_keys = ON"); err != nil {
		return nil, err
	}

	return &DB{db}, nil
}

func (db *DB) Close() {
	db.DB.Close()
}

func (db *DB) InitTables() error {
	if _, err := db.Exec(`CREATE TABLE IF NOT EXISTS users (
	  id INTEGER PRIMARY KEY AUTOINCREMENT,
	  name TEXT NOT NULL,
	  email TEXT NOT NULL UNIQUE,
	  password_hash TEXT NOT NULL,
	  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`); err != nil {
		return err
	}

	if _, err := db.Exec(`CREATE TABLE IF NOT EXISTS expenses (
	  id INTEGER PRIMARY KEY AUTOINCREMENT,
	  user_id INTEGER NOT NULL,
	  amount REAL NOT NULL,
	  category TEXT,
	  description TEXT,
	  date DATE NOT NULL,
	  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
	)`); err != nil {
		return err
	}

	return nil
}

func (db *DB) ClearAllTables() error {
	if _, err := db.Exec(`DELETE FROM expenses`); err != nil {
		return err
	}

	if _, err := db.Exec(`DELETE FROM users`); err != nil {
		return err
	}

	return nil
}

func (db *DB) RegisterUser(ctx context.Context, params CreateUserParams) (*DBUser, error) {
	query := `
		INSERT INTO users (name, email, password_hash)
		VALUES (?, ?, ?)
		RETURNING id, name, email, created_at
	`

	hashedPassword, err := HashPassword(params.Password)
	if err != nil {
		return nil, err
	}

	row := db.QueryRowContext(ctx, query, params.Name, params.Email, hashedPassword)

	var user DBUser
	err = row.Scan(&user.ID, &user.Name, &user.Email, &user.CreatedAt)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

func (db *DB) LoginUser(ctx context.Context, params LoginParams) (*DBUser, error) {
	query := `
		SELECT id, name, email, password_hash, created_at
		FROM users
		WHERE email = ?
	`

	row := db.QueryRowContext(ctx, query, params.Email)

	var user DBUser
	err := row.Scan(&user.ID, &user.Name, &user.Email, &user.PasswordHash, &user.CreatedAt)
	if err != nil {
		return nil, err
	}

	if !CheckPasswordHash(params.Password, user.PasswordHash) {
		return nil, sql.ErrNoRows
	}

	return &user, nil
}

func (db *DB) CreateExpense(ctx context.Context, params CreateExpenseParams) (*DBExpense, error) {
	query := `
		INSERT INTO expenses (user_id, amount, category, description, date)
		VALUES (?, ?, ?, ?, ?)
		RETURNING id, user_id, amount, category, description, date
	`

	row := db.QueryRowContext(ctx, query, params.UserID, params.Amount, params.Category, params.Description, params.Date)

	var expense DBExpense
	err := row.Scan(&expense.ID, &expense.UserID, &expense.Amount, &expense.Category, &expense.Description, &expense.Date)
	if err != nil {
		return nil, err
	}

	return &expense, nil
}

func (db *DB) GetExpenseByID(ctx context.Context, id int64) (*DBExpense, error) {
	query := `
		SELECT id, user_id, amount, category, description, date
		FROM expenses
		WHERE id = ?
	`

	row := db.QueryRowContext(ctx, query, id)

	var expense DBExpense
	err := row.Scan(&expense.ID, &expense.UserID, &expense.Amount, &expense.Category, &expense.Description, &expense.Date)
	if err != nil {
		return nil, err
	}

	return &expense, nil
}

func (db *DB) UpdateExpense(ctx context.Context, params UpdateExpenseParams) (*DBExpense, error) {
	query := `
		UPDATE expenses
		SET amount = COALESCE(?, amount),
			category = COALESCE(?, category),
			description = COALESCE(?, description),
			date = COALESCE(?, date)
		WHERE id = ? AND user_id = ?
		RETURNING id, user_id, amount, category, description, date
	`

	row := db.QueryRowContext(ctx, query,
		params.Amount, params.Category, params.Description, params.Date,
		params.ID, params.UserID,
	)

	var expense DBExpense
	err := row.Scan(&expense.ID, &expense.UserID, &expense.Amount, &expense.Category, &expense.Description, &expense.Date)
	if err != nil {
		return nil, err
	}

	return &expense, nil
}

func (db *DB) DeleteExpense(ctx context.Context, id int64, userID int64) (*DBExpense, error) {
	query := `
		DELETE FROM expenses
		WHERE id = ? AND user_id = ?
		RETURNING id, user_id, amount, category, description, date
	`

	row := db.QueryRowContext(ctx, query, id, userID)

	var expense DBExpense
	err := row.Scan(&expense.ID, &expense.UserID, &expense.Amount, &expense.Category, &expense.Description, &expense.Date)
	if err != nil {
		return nil, err
	}

	return &expense, nil
}

func (db *DB) GetAllExpenses(ctx context.Context, params GetExpenseParams) ([]DBExpense, error) {
	query := `
		SELECT id, user_id, amount, category, description, date
		FROM expenses
		WHERE user_id = ?
	`

	args := []any{params.UserID}

	startDate := time.Time{}
	endDate := time.Time{}

	switch params.Period {
	case Custom:
		if params.StartDate != nil && params.EndDate != nil {
			startDate, _ = time.Parse(time.RFC3339, *params.StartDate)
			endDate, _ = time.Parse(time.RFC3339, *params.EndDate)
		} else {
			return nil, errors.New("start date and end date are required for custom period")
		}

	case PastMonth:
		endDate = time.Now()
		startDate = endDate.AddDate(0, -1, 0)
	case Past3Month:
		endDate = time.Now()
		startDate = endDate.AddDate(0, -3, 0)
	case PastWeek:
		fallthrough
	default:
		endDate = time.Now()
		startDate = endDate.AddDate(0, 0, -7)
	}

	if startDate != endDate {
		query += " AND date BETWEEN ? AND ?"
		args = append(args, startDate.Format("2006-01-02"), endDate.Format("2006-01-02"))
	}

	if params.Category != nil {
		query += " AND category = ?"
		args = append(args, *params.Category)
	}

	rows, err := db.QueryContext(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var expenses []DBExpense
	for rows.Next() {
		var expense DBExpense
		err := rows.Scan(&expense.ID, &expense.UserID, &expense.Amount, &expense.Category, &expense.Description, &expense.Date)
		if err != nil {
			return nil, err
		}
		expenses = append(expenses, expense)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	return expenses, nil
}
