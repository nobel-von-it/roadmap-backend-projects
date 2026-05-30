package main

import (
	"context"
	"database/sql"
	"strconv"

	"github.com/gofiber/fiber/v3"
)

type Env struct {
	db Storage
}

func NewEnv(db Storage) *Env {
	return &Env{db}
}

func (e *Env) RegisterUserHandler(c fiber.Ctx) error {
	var params CreateUserParams
	if err := c.Bind().Body(&params); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	if params.Name == "" || params.Email == "" || params.Password == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "All fields are required",
		})
	}

	hashedPassword, err := HashPassword(params.Password)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	params.Password = hashedPassword

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	user, err := e.db.RegisterUser(ctx, params)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	token, err := GenerateJWT(*user)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusCreated).JSON(fiber.Map{
		"token": token,
	})
}

func (e *Env) LoginUserHandler(c fiber.Ctx) error {
	var params LoginParams
	if err := c.Bind().Body(&params); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	if params.Email == "" || params.Password == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "All fields are required",
		})
	}

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	user, err := e.db.LoginUser(ctx, params)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	if !CheckPasswordHash(params.Password, user.PasswordHash) {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Invalid password",
		})
	}

	token, err := GenerateJWT(*user)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"token": token,
	})
}

func (e *Env) CreateExpenseHandler(c fiber.Ctx) error {
	var req CreateExpenseRequest
	if err := c.Bind().Body(&req); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	if req.Amount == 0 || req.Date == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Amount and Date fields are required",
		})
	}

	userID := c.Locals("userID")
	if userID == nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Failed to get user ID",
		})
	}

	params := CreateExpenseParams{
		UserID:      userID.(int64),
		Amount:      req.Amount,
		Category:    req.Category,
		Description: req.Description,
		Date:        req.Date,
	}

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	expense, err := e.db.CreateExpense(ctx, params)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusCreated).JSON(fiber.Map{
		"expense": expense,
	})
}

func (e *Env) UpdateExpenseHandler(c fiber.Ctx) error {
	var req UpdateExpenseRequest
	if err := c.Bind().Body(&req); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	if req.ID == 0 {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Expense ID is required",
		})
	}

	userID := c.Locals("userID")
	if userID == nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Failed to get user ID",
		})
	}

	params := UpdateExpenseParams{
		ID:          req.ID,
		UserID:      userID.(int64),
		Amount:      req.Amount,
		Category:    req.Category,
		Description: req.Description,
		Date:        req.Date,
	}

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	expense, err := e.db.UpdateExpense(ctx, params)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"expense": expense,
	})
}

func (e *Env) DeleteExpenseHandler(c fiber.Ctx) error {
	idStr := c.Params("id")
	if idStr == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Expense ID is required",
		})
	}

	id, err := strconv.ParseInt(idStr, 10, 64)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Invalid expense ID",
		})
	}

	userID := c.Locals("userID")
	if userID == nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Failed to get user ID",
		})
	}

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	expense, err := e.db.DeleteExpense(ctx, id, userID.(int64))
	if err != nil {
		if err == sql.ErrNoRows {
			return c.Status(fiber.StatusNotFound).JSON(fiber.Map{
				"error": "Expense not found or you are not authorized to delete it",
			})
		}
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"expense": expense,
	})
}

func (e *Env) GetExpensesHandler(c fiber.Ctx) error {
	var req GetExpenseRequest
	if err := c.Bind().Query(&req); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	userID := c.Locals("userID")
	if userID == nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Failed to get user ID",
		})
	}

	params := GetExpenseParams{
		UserID:    userID.(int64),
		Category:  req.Category,
		Period:    req.Period,
		StartDate: req.StartDate,
		EndDate:   req.EndDate,
	}

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	expenses, err := e.db.GetAllExpenses(ctx, params)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"expenses": expenses,
	})
}
