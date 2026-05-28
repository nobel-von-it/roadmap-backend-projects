package main_test

import (
	"database/sql"
	. "nobel/todo-api"
	"testing"

	"github.com/stretchr/testify/require"
)

func setupTestDb(t *testing.T) *DB {
	t.Helper()

	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	if _, err := sqliteDB.Exec("PRAGMA foreign_keys = ON"); err != nil {
		sqliteDB.Close()
		t.Fatalf("Failed to enable foreign keys: %v", err)
	}

	db := &DB{DB: sqliteDB}
	db.InitTables()
	t.Cleanup(func() {
		db.Close()
	})

	return db
}

func TestDB_AddAndGetTodo(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	todo, err := db.AddTodo("test", "test", user.ID)
	require.NoError(t, err)
	require.NotNil(t, todo)

	require.Equal(t, todo.Title, "test")
	require.Equal(t, todo.Description, "test")
	require.Equal(t, todo.Completed, false)
	require.Equal(t, todo.UserID, user.ID)

	fetchedTodos, err := db.GetTodos(user.ID, 1, 10)
	require.NoError(t, err)
	require.Len(t, fetchedTodos, 1)
	require.Equal(t, todo.ID, fetchedTodos[0].ID)
	require.Equal(t, todo.Title, fetchedTodos[0].Title)
	require.Equal(t, todo.Description, fetchedTodos[0].Description)
	require.Equal(t, todo.Completed, fetchedTodos[0].Completed)
	require.Equal(t, todo.UserID, fetchedTodos[0].UserID)
}

func TestDB_AddExistingTodo(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	todo, err := db.AddTodo("test", "test", user.ID)
	require.NoError(t, err)
	require.NotNil(t, todo)

	todo2, err := db.AddTodo("test", "test", user.ID)
	require.NoError(t, err)
	require.NotNil(t, todo2)

	fetchedTodos, err := db.GetTodos(user.ID, 1, 10)
	require.NoError(t, err)
	require.Len(t, fetchedTodos, 2)
	require.Equal(t, fetchedTodos[0].ID, todo.ID)
	require.Equal(t, fetchedTodos[1].ID, todo2.ID)
	require.NotEqual(t, fetchedTodos[0].ID, fetchedTodos[1].ID)
}

func TestDB_AddExistingUser(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	user2, err := db.Register("test", "test@example.com", []byte("test"))
	require.Error(t, err)
	require.Nil(t, user2)
}

func TestDB_DeleteTodo(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	todo, err := db.AddTodo("test", "test", user.ID)
	require.NoError(t, err)
	require.NotNil(t, todo)

	todo2, err := db.AddTodo("test2", "test2", user.ID)
	require.NoError(t, err)
	require.NotNil(t, todo2)

	fetchedTodos, err := db.GetTodos(user.ID, 1, 10)
	require.NoError(t, err)
	require.Len(t, fetchedTodos, 2)

	err = db.DeleteTodo(todo.ID, user.ID)
	require.NoError(t, err)

	fetchedTodos, err = db.GetTodos(user.ID, 1, 10)
	require.NoError(t, err)
	require.Len(t, fetchedTodos, 1)
	require.Equal(t, fetchedTodos[0].ID, todo2.ID)
	require.NotEqual(t, fetchedTodos[0].ID, todo.ID)
}

func TestDB_DeleteTodoNonExisting(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	err = db.DeleteTodo(100, user.ID)
	require.Error(t, err)
}

func TestDB_GetEmptyTodos(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	fetchedTodos, err := db.GetTodos(user.ID, 1, 10)
	require.NoError(t, err)
	require.Len(t, fetchedTodos, 0)
}

func TestDB_GetTodosNonExistingUser(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	fetchedTodos, err := db.GetTodos(100, 1, 10)
	require.NoError(t, err)
	require.Len(t, fetchedTodos, 0)
}

func TestDB_CreateTodoNonExistingUser(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	todo, err := db.AddTodo("test", "test", 100)
	require.Error(t, err)
	require.Nil(t, todo)
}

func TestDB_UpdateTodo(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	todo, err := db.AddTodo("test", "test", user.ID)
	require.NoError(t, err)
	require.NotNil(t, todo)

	fetchedTodos, err := db.GetTodos(user.ID, 1, 10)
	require.NoError(t, err)
	require.Len(t, fetchedTodos, 1)
	require.Equal(t, fetchedTodos[0].Completed, false)

	err = db.UpdateTodo(todo.ID, "test", "test", true, user.ID)
	require.NoError(t, err)

	fetchedTodos, err = db.GetTodos(user.ID, 1, 10)
	require.NoError(t, err)
	require.Len(t, fetchedTodos, 1)
	require.Equal(t, fetchedTodos[0].Completed, true)
}

func TestDB_UpdateTodoNonExisting(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	err = db.UpdateTodo(100, "test", "test", true, user.ID)
	require.Error(t, err)
}

func TestDB_UpdateTodoNonExistingUser(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	err = db.UpdateTodo(1, "test", "test", true, 100)
	require.Error(t, err)
}

func TestDB_UpdateTodoWrongUser(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)
	require.NotNil(t, user)

	user2, err := db.Register("test2", "test2@example.com", []byte("test2"))
	require.NoError(t, err)
	require.NotNil(t, user2)

	todo, err := db.AddTodo("test", "test", user.ID)
	require.NoError(t, err)
	require.NotNil(t, todo)

	err = db.UpdateTodo(todo.ID, "test", "test", true, user2.ID)
	require.Error(t, err)
}

func TestDB_GetTodosPagination(t *testing.T) {
	db := setupTestDb(t)

	user, err := db.Register("test", "test@example.com", []byte("test"))
	require.NoError(t, err)

	_, err = db.AddTodo("1", "desc1", user.ID)
	require.NoError(t, err)
	_, err = db.AddTodo("2", "desc2", user.ID)
	require.NoError(t, err)
	_, err = db.AddTodo("3", "desc3", user.ID)
	require.NoError(t, err)

	todos1, err := db.GetTodos(user.ID, 1, 2)
	require.NoError(t, err)
	require.Len(t, todos1, 2)
	require.Equal(t, "1", todos1[0].Title)
	require.Equal(t, "2", todos1[1].Title)

	todos2, err := db.GetTodos(user.ID, 2, 2)
	require.NoError(t, err)
	require.Len(t, todos2, 1)
	require.Equal(t, "3", todos2[0].Title)
}
