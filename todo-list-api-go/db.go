package main

import (
	"database/sql"

	_ "modernc.org/sqlite"
)

type DB struct {
	*sql.DB
}

func NewDB() (*DB, error) {
	db, err := sql.Open("sqlite", "./todo.db")
	if err != nil {
		return nil, err
	}
	return &DB{db}, nil
}

func (db *DB) Close() {
	db.DB.Close()
}

func (db *DB) InitTables() {
	db.Exec(`CREATE TABLE IF NOT EXISTS users (
		ID INTEGER PRIMARY KEY AUTOINCREMENT,
		Name TEXT NOT NULL,
		Email TEXT UNIQUE NOT NULL,
		Password TEXT NOT NULL
	)`)
	db.Exec(`CREATE TABLE IF NOT EXISTS todos (
		ID INTEGER PRIMARY KEY AUTOINCREMENT,
		Title TEXT NOT NULL,
		Description TEXT,
		Completed BOOLEAN,
		UserID INTEGER NOT NULL,
		FOREIGN KEY (UserID) REFERENCES users (ID)
	)`)
}

func (db *DB) ClearTables() {
	db.Exec("DROP TABLE IF EXISTS todos")
	db.Exec("DROP TABLE IF EXISTS users")
}

func (db *DB) Register(name string, email string, password []byte) (*DbUser, error) {
	stmt, err := db.Prepare("INSERT INTO users (name, email, password) VALUES (?, ?, ?)")
	if err != nil {
		return nil, err
	}
	defer stmt.Close()
	result, err := stmt.Exec(name, email, password)
	if err != nil {
		return nil, err
	}
	userId, err := result.LastInsertId()
	if err != nil {
		return nil, err
	}

	return &DbUser{ID: int(userId), User: User{Name: name, Password: password}}, nil
}

func (db *DB) Login(email string, password []byte) (*DbUser, error) {
	stmt, err := db.Prepare("SELECT * FROM users WHERE email = ? AND password = ?")
	if err != nil {
		return nil, err
	}
	defer stmt.Close()
	var user DbUser
	err = stmt.QueryRow(email, password).Scan(&user.ID, &user.Name, &user.Email, &user.Password)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

func (db *DB) AddTodo(title, description string, userId int) (*DbTodo, error) {
	stmt, err := db.Prepare("INSERT INTO todos (title, description, completed, userId) VALUES (?, ?, ?, ?)")
	if err != nil {
		return nil, err
	}
	defer stmt.Close()
	result, err := stmt.Exec(title, description, false, userId)
	if err != nil {
		return nil, err
	}
	todoId, err := result.LastInsertId()
	if err != nil {
		return nil, err
	}

	return &DbTodo{ID: int(todoId), Todo: Todo{Title: title, Description: description, Completed: false, UserID: userId}}, nil
}

func (db *DB) DeleteTodo(todoID int, userID int) error {
	stmt, err := db.Prepare("DELETE FROM todos WHERE id = ? AND userId = ?")
	if err != nil {
		return err
	}
	defer stmt.Close()
	_, err = stmt.Exec(todoID, userID)
	return err
}

func (db *DB) UpdateTodo(id int, title, description string, completed bool, userId int) error {
	stmt, err := db.Prepare("UPDATE todos SET title = ?, description = ?, completed = ? WHERE id = ? AND userId = ?")
	if err != nil {
		return err
	}
	defer stmt.Close()
	_, err = stmt.Exec(title, description, completed, id, userId)
	return err
}

func (db *DB) GetTodos(userId int) ([]DbTodo, error) {
	stmt, err := db.Prepare("SELECT * FROM todos WHERE userId = ?")
	if err != nil {
		return nil, err
	}
	defer stmt.Close()
	rows, err := stmt.Query(userId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var todos []DbTodo
	for rows.Next() {
		var todo DbTodo
		err = rows.Scan(&todo.ID, &todo.Title, &todo.Description, &todo.Completed, &todo.UserID)
		if err != nil {
			return nil, err
		}
		todos = append(todos, todo)
	}
	return todos, nil
}
