package main

import (
	"context"
	"time"
)

type Storage interface {
	UserRepository
	ExpenseRepository
}

type UserRepository interface {
	RegisterUser(ctx context.Context, params CreateUserParams) (*DBUser, error)
	LoginUser(ctx context.Context, params LoginParams) (*DBUser, error)
}

type ExpenseRepository interface {
	CreateExpense(ctx context.Context, params CreateExpenseParams) (*DBExpense, error)
	GetExpenseByID(ctx context.Context, id int64) (*DBExpense, error)
	UpdateExpense(ctx context.Context, params UpdateExpenseParams) (*DBExpense, error)
	DeleteExpense(ctx context.Context, id int64, userID int64) (*DBExpense, error)
	GetAllExpenses(ctx context.Context, params GetExpenseParams) ([]DBExpense, error)
}

type CreateUserParams struct {
	Name     string `json:"name"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

type LoginParams struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

type DBUser struct {
	ID           int64  `json:"id"`
	Name         string `json:"name"`
	Email        string `json:"email"`
	PasswordHash string `json:"-"`
	CreatedAt    string `json:"created_at"`
}

type CreateExpenseRequest struct {
	Amount      float64 `json:"amount"`
	Category    *string `json:"category"`
	Description *string `json:"description"`
	Date        string  `json:"date"`
}

type CreateExpenseParams struct {
	UserID      int64
	Amount      float64
	Category    *string
	Description *string
	Date        string
}

type UpdateExpenseRequest struct {
	ID          int64    `json:"id"`
	Amount      *float64 `json:"amount"`
	Category    *string  `json:"category"`
	Description *string  `json:"description"`
	Date        *string  `json:"date"`
}

type UpdateExpenseParams struct {
	ID          int64
	UserID      int64
	Amount      *float64
	Category    *string
	Description *string
	Date        *string
}

type GetExpenseRequest struct {
	Category  *CategoryFilter `json:"category"`
	Period    PeriodFilter    `json:"period"`
	StartDate *string         `json:"start_date"`
	EndDate   *string         `json:"end_date"`
}

type GetExpenseParams struct {
	UserID    int64
	Category  *CategoryFilter
	Period    PeriodFilter
	StartDate *string
	EndDate   *string
}

type DBExpense struct {
	ID          int64     `json:"id"`
	UserID      int64     `json:"user_id"`
	Amount      float64   `json:"amount"`
	Category    *string   `json:"category"`
	Description *string   `json:"description"`
	Date        time.Time `json:"date"`
}

type PeriodFilter string

const (
	PastWeek   PeriodFilter = "past_week"
	PastMonth  PeriodFilter = "past_month"
	Past3Month PeriodFilter = "past_3_months"
	Custom     PeriodFilter = "custom"
)

type CategoryFilter string

const (
	Groceries   CategoryFilter = "groceries"
	Leisure     CategoryFilter = "leisure"
	Electronics CategoryFilter = "electronics"
	Utilities   CategoryFilter = "utilities"
	Clothing    CategoryFilter = "clothing"
	Health      CategoryFilter = "health"
	Others      CategoryFilter = "others"
)
