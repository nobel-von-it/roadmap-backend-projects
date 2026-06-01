package main

import (
	"context"
	"strconv"

	"github.com/gofiber/fiber/v3"
)

type Env struct {
	db *DBController
}

func NewEnv(db *DBController) *Env {
	return &Env{db}
}

func (e *Env) CreateNoteHandler(c fiber.Ctx) error {
	var createNoteParams CreateNoteParams
	if err := c.Bind().Body(&createNoteParams); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	note, err := e.db.CreateNote(ctx, createNoteParams)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	note.Content = MdRender(note.Content)

	return c.Status(fiber.StatusCreated).JSON(fiber.Map{
		"note": note,
	})
}

func (e *Env) RenderNoteHandler(c fiber.Ctx) error {
	idStr := c.Params("id")

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	id, err := strconv.ParseInt(idStr, 10, 64)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	note, err := e.db.GetNoteByID(ctx, id)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	note.Content = MdRender(note.Content)

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"note": note,
	})
}
