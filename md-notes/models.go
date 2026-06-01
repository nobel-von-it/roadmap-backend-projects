package main

import (
	"context"
	"time"
)

type Storage interface {
	NoteRepository
}

type NoteRepository interface {
	CreateNote(ctx context.Context, params CreateNoteParams) (*DBNote, error)
	GetNoteByID(ctx context.Context, id int64) (*DBNote, error)
	UpdateNote(ctx context.Context, params UpdateNoteParams) (*DBNote, error)
	DeleteNote(ctx context.Context, id int64) (*DBNote, error)
	GetNotes(ctx context.Context, params GetNoteParams) ([]DBNote, error)
}

type CreateNoteParams struct {
	Note
}

type UpdateNoteParams struct {
	ID int64 `json:"id"`
	Note
}

type GetNoteParams struct {
	Query  string `json:"query"`
	Limit  int64  `json:"limit"`
	Offset int64  `json:"offset"`
}

type DBNote struct {
	ID        int64
	Title     string
	Content   string
	CreatedAt time.Time
	UpdatedAt time.Time
}

type Note struct {
	Title   string `json:"title"`
	Content string `json:"content"`
}
