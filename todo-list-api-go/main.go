package main

import "github.com/gofiber/fiber/v3"

func main() {
	app := fiber.New()

	db, err := NewDB()
	if err != nil {
		panic(err)
	}
	db.InitTables()
	defer db.Close()

	env := NewEnv(db)

	app.Post("/register", env.RegisterHandler)
	app.Post("/login", env.LoginHandler)
	app.Post("/todos", AuthMiddleware, env.CreateTodoHandler)
	app.Patch("/todos/:id", AuthMiddleware, env.UpdateTodoHandler)
	app.Delete("/todos/:id", AuthMiddleware, env.DeleteTodoHandler)
	app.Get("/todos", AuthMiddleware, env.GetTodosHandler)

	app.Listen(":8080")
}
