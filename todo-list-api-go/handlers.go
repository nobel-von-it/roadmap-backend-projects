package main

import (
	"strconv"

	"github.com/gofiber/fiber/v3"
)

type Env struct {
	db Storage
}

func NewEnv(db Storage) *Env {
	return &Env{db}
}

func (env *Env) RegisterHandler(c fiber.Ctx) error {
	var user User
	if err := c.Bind().Body(&user); err != nil {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}
	if user.Email == "" || user.Name == "" || user.Password == nil {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}

	dbUser, err := env.db.Register(user.Name, user.Email, user.Password)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"message": "Failed to register user",
		})
	}

	token, err := GenerateJwt(*dbUser)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"message": "Failed to generate token",
		})
	}

	return c.JSON(fiber.Map{
		"token": token,
	})
}

func (env *Env) LoginHandler(c fiber.Ctx) error {
	var user User
	if err := c.Bind().Body(&user); err != nil {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}
	if user.Email == "" || user.Password == nil {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}

	dbUser, err := env.db.Login(user.Email, user.Password)
	if err != nil {
		return c.Status(401).JSON(fiber.Map{
			"message": "Failed to login",
		})
	}

	token, err := GenerateJwt(*dbUser)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"message": "Failed to generate token",
		})
	}

	return c.JSON(fiber.Map{
		"token": token,
	})
}

func (env *Env) CreateTodoHandler(c fiber.Ctx) error {
	var todo Todo
	if err := c.Bind().Body(&todo); err != nil {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}
	if todo.Title == "" {
		return c.Status(400).JSON(fiber.Map{
			"message": "Title is required",
		})
	}

	userIdVal := c.Locals("userId")
	if userIdVal == nil {
		return c.Status(401).JSON(fiber.Map{
			"message": "Unauthorized",
		})
	}
	userID := userIdVal.(int)

	newTodo, err := env.db.AddTodo(todo.Title, todo.Description, userID)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"message": "Failed to create todo",
		})
	}

	return c.Status(201).JSON(fiber.Map{
		"message": "Todo created successfully",
		"todo":    newTodo,
	})
}

func (env *Env) UpdateTodoHandler(c fiber.Ctx) error {
	todoIDVal := c.Params("id")
	if todoIDVal == "" {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}
	todoID, _ := strconv.Atoi(todoIDVal)
	var todo Todo
	if err := c.Bind().Body(&todo); err != nil {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}
	if todo.Title == "" || todo.Description == "" {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}

	userIdVal := c.Locals("userId")
	if userIdVal == nil {
		return c.Status(401).JSON(fiber.Map{
			"message": "Unauthorized",
		})
	}
	userID := userIdVal.(int)

	if err := env.db.UpdateTodo(todoID, todo.Title, todo.Description, todo.Completed, userID); err != nil {
		return c.Status(500).JSON(fiber.Map{
			"message": "Failed to update todo",
		})
	}

	return c.Status(201).JSON(fiber.Map{
		"message": "Todo updated successfully",
	})
}

func (env *Env) GetTodosHandler(c fiber.Ctx) error {
	userIdVal := c.Locals("userId")
	if userIdVal == nil {
		return c.Status(401).JSON(fiber.Map{
			"message": "Unauthorized",
		})
	}
	userID := userIdVal.(int)

	pageVal := c.Query("page")
	var page int
	if pageVal == "" {
		page = 1
	} else {
		page, _ = strconv.Atoi(pageVal)
	}

	limitVal := c.Query("limit")
	var limit int
	if limitVal == "" {
		limit = 10
	} else {
		limit, _ = strconv.Atoi(limitVal)
	}

	todos, err := env.db.GetTodos(userID, page, limit)
	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"message": "Failed to get todos",
		})
	}

	return c.JSON(fiber.Map{
		"todos": todos,
		"meta": map[string]int{
			"page":  page,
			"limit": limit,
			"total": len(todos),
		},
	})
}

func (env *Env) DeleteTodoHandler(c fiber.Ctx) error {
	todoIDVal := c.Params("id")
	if todoIDVal == "" {
		return c.Status(400).JSON(fiber.Map{
			"message": "Invalid request",
		})
	}
	todoID, _ := strconv.Atoi(todoIDVal)

	userIdVal := c.Locals("userId")
	if userIdVal == nil {
		return c.Status(401).JSON(fiber.Map{
			"message": "Unauthorized",
		})
	}
	userID := userIdVal.(int)

	if err := env.db.DeleteTodo(todoID, userID); err != nil {
		return c.Status(500).JSON(fiber.Map{
			"message": "Failed to delete todo",
		})
	}

	return c.Status(201).JSON(fiber.Map{
		"message": "Todo deleted successfully",
	})
}
