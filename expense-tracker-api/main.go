package main

import (
	"log"

	"github.com/gofiber/fiber/v3"
)

func main() {
	app := fiber.New()

	db, err := NewDB()
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	if err := db.InitTables(); err != nil {
		log.Fatal(err)
	}

	env := NewEnv(db)

	api := app.Group("/api")
	api.Post("/auth/register", env.RegisterUserHandler)
	api.Post("/auth/login", env.LoginUserHandler)

	api.Use(AuthMiddleware)

	expenses := api.Group("/expenses")
	expenses.Get("/", env.GetExpensesHandler)
	expenses.Post("/", env.CreateExpenseHandler)
	expenses.Put("/", env.UpdateExpenseHandler)
	expenses.Delete("/:id", env.DeleteExpenseHandler)

	log.Println("Starting server on :8080...")

	app.Listen(":8080")
}
