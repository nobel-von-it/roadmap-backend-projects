package main

import "encoding/json"

type UserRepository interface {
	Register(name string, email string, password []byte) (*DbUser, error)
	Login(email string, password []byte) (*DbUser, error)
}

type TodoRepository interface {
	AddTodo(title, description string, userId int) (*DbTodo, error)
	DeleteTodo(todoID int, userID int) error
	UpdateTodo(id int, title, description string, completed bool, userId int) error
	GetTodos(userId int) ([]DbTodo, error)
}

type Storage interface {
	UserRepository
	TodoRepository
}

type DbUser struct {
	ID int
	User
}

type User struct {
	Name     string `json:"name"`
	Email    string `json:"email"`
	Password []byte `json:"password"`
}

func (u *User) UnmarshalJSON(data []byte) error {
	type Alias User

	aux := &struct {
		Password string `json:"password"`
		*Alias
	}{
		Alias: (*Alias)(u),
	}

	if err := json.Unmarshal(data, aux); err != nil {
		return err
	}
	u.Password = []byte(aux.Password)
	return nil
}

type DbTodo struct {
	ID int
	Todo
}

type Todo struct {
	Title       string `json:"title"`
	Description string `json:"description"`
	Completed   bool   `json:"completed"`
	UserID      int    `json:"user_id"`
}
