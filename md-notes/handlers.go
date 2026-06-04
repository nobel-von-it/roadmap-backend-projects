package main

import (
	"context"
	"nobel/md-notes/api"
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

func (e *Env) CheckNoteHandler(c fiber.Ctx) error {
	var checkNoteParams api.LTRequest
	checkNoteIDStr := c.Params("id")

	ctx, cancel := context.WithTimeout(c.Context(), ContextTimeout)
	defer cancel()

	checkNoteID, err := strconv.ParseInt(checkNoteIDStr, 10, 64)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	note, err := e.db.GetNoteByID(ctx, checkNoteID)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	checkNoteParams.Text = note.Content
	checkNoteParams.Language = api.Auto

	result, err := checkNoteParams.Fetch(ctx)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"result": result,
	})
}
