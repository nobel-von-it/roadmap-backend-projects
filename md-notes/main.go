package main

import (
	"log"

	"github.com/gofiber/fiber/v3"
)

func main() {
	db, err := NewDB()
	if err != nil {
		log.Fatal("failed to connect to database: ", err)
	}

	if err := db.InitTables(); err != nil {
		log.Fatal(err)
	}

	dbController := NewDBController(db)
	env := NewEnv(dbController)

	app := fiber.New()

	api := app.Group("/api")

	api.Post("/notes", env.CreateNoteHandler)
	api.Get("/notes/:id/render", env.RenderNoteHandler)

	app.Listen(":8080")
}
