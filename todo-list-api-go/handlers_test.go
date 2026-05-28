package main_test

import (
	"bytes"
	"encoding/json"
	"net/http/httptest"
	. "nobel/todo-api"
	"testing"

	"github.com/gofiber/fiber/v3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

type MockStorage struct {
	mock.Mock
}

func (m *MockStorage) Register(name string, email string, password []byte) (*DbUser, error) {
	args := m.Called(name, email, password)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*DbUser), args.Error(1)
}

func (m *MockStorage) Login(email string, password []byte) (*DbUser, error) {
	args := m.Called(email, password)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*DbUser), args.Error(1)
}

func (m *MockStorage) AddTodo(title, description string, userId int) (*DbTodo, error) {
	args := m.Called(title, description, userId)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*DbTodo), args.Error(1)
}

func (m *MockStorage) DeleteTodo(todoID int, userId int) error {
	return nil
}

func (m *MockStorage) UpdateTodo(id int, title, description string, completed bool, userId int) error {
	return nil
}

func (m *MockStorage) GetTodos(userId int, page int, limit int) ([]DbTodo, error) {
	return nil, nil
}

func TestCreateTodoHandler(t *testing.T) {
	t.Run("successful request", func(t *testing.T) {
		app := fiber.New()
		mockStorage := new(MockStorage)
		env := NewEnv(mockStorage)

		expectedTodo := &DbTodo{
			ID: 42,
			Todo: Todo{
				Title:       "title",
				Description: "description",
				Completed:   false,
				UserID:      100,
			},
		}
		mockStorage.On("AddTodo", "title", "description", 100).Return(expectedTodo, nil)

		app.Post("/todos", func(c fiber.Ctx) error {
			c.Locals("userId", 100)
			return c.Next()
		}, env.CreateTodoHandler)

		reqBody, _ := json.Marshal(Todo{
			Title:       "title",
			Description: "description",
		})

		req := httptest.NewRequest("POST", "/todos", bytes.NewBuffer(reqBody))
		req.Header.Set("Content-Type", "application/json")

		resp, err := app.Test(req)
		require.NoError(t, err)

		defer resp.Body.Close()

		assert.Equal(t, 201, resp.StatusCode)

		var body map[string]interface{}
		err = json.NewDecoder(resp.Body).Decode(&body)
		require.NoError(t, err)

		assert.Equal(t, "Todo created successfully", body["message"])

		mockStorage.AssertExpectations(t)
	})

	t.Run("failed with empty title", func(t *testing.T) {
		app := fiber.New()
		mockStorage := new(MockStorage)
		env := NewEnv(mockStorage)

		app.Post("/todos", func(c fiber.Ctx) error {
			c.Locals("userId", 100)
			return c.Next()
		}, env.CreateTodoHandler)

		reqBody, _ := json.Marshal(Todo{
			Title:       "",
			Description: "description",
		})

		req := httptest.NewRequest("POST", "/todos", bytes.NewBuffer(reqBody))
		req.Header.Set("Content-Type", "application/json")

		resp, err := app.Test(req)
		require.NoError(t, err)

		defer resp.Body.Close()

		assert.Equal(t, 400, resp.StatusCode)
	})

	t.Run("successful with empty description", func(t *testing.T) {
		app := fiber.New()
		mockStorage := new(MockStorage)
		env := NewEnv(mockStorage)

		app.Post("/todos", func(c fiber.Ctx) error {
			c.Locals("userId", 100)
			return c.Next()
		}, env.CreateTodoHandler)

		reqBody, _ := json.Marshal(Todo{
			Title:       "title",
			Description: "",
		})
		mockStorage.On("AddTodo", "title", "", 100).Return(&DbTodo{
			ID: 42,
			Todo: Todo{
				Title:       "title",
				Description: "",
				Completed:   false,
				UserID:      100,
			},
		}, nil)

		req := httptest.NewRequest("POST", "/todos", bytes.NewBuffer(reqBody))
		req.Header.Set("Content-Type", "application/json")

		resp, err := app.Test(req)
		require.NoError(t, err)

		defer resp.Body.Close()

		assert.Equal(t, 201, resp.StatusCode)

		var body map[string]interface{}
		err = json.NewDecoder(resp.Body).Decode(&body)
		require.NoError(t, err)

		assert.Equal(t, "Todo created successfully", body["message"])

		mockStorage.AssertExpectations(t)
	})
}
