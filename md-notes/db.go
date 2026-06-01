package main

import (
	"context"
	"database/sql"

	_ "modernc.org/sqlite"
)

type DBController struct {
	db Storage
}

func NewDBController(db Storage) *DBController {
	return &DBController{db: db}
}

func (d *DBController) CreateNote(ctx context.Context, params CreateNoteParams) (*DBNote, error) {
	return d.db.CreateNote(ctx, params)
}

func (d *DBController) GetNoteByID(ctx context.Context, id int64) (*DBNote, error) {
	return d.db.GetNoteByID(ctx, id)
}

func (d *DBController) UpdateNote(ctx context.Context, params UpdateNoteParams) (*DBNote, error) {
	return d.db.UpdateNote(ctx, params)
}

func (d *DBController) DeleteNote(ctx context.Context, id int64) (*DBNote, error) {
	return d.db.DeleteNote(ctx, id)
}

func (d *DBController) GetNotes(ctx context.Context, params GetNoteParams) ([]DBNote, error) {
	return d.db.GetNotes(ctx, params)
}

type DB struct {
	*sql.DB
}

func NewDB() (*DB, error) {
	db, err := sql.Open("sqlite", "./notes.db")
	if err != nil {
		return nil, err
	}

	if err := db.Ping(); err != nil {
		return nil, err
	}

	return &DB{db}, nil
}

func (db *DB) InitTables() error {
	_, err := db.Exec(`CREATE TABLE IF NOT EXISTS notes (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		title TEXT NOT NULL,
		content TEXT NOT NULL,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	return err
}

func (db *DB) CreateNote(ctx context.Context, createNoteParams CreateNoteParams) (*DBNote, error) {
	query := `
		INSERT INTO notes (title, content)
		VALUES (?, ?)
		RETURNING id, title, content, created_at, updated_at
	`

	row := db.QueryRowContext(ctx, query, createNoteParams.Title, createNoteParams.Content)
	var newNote DBNote
	err := row.Scan(
		&newNote.ID,
		&newNote.Title,
		&newNote.Content,
		&newNote.CreatedAt,
		&newNote.UpdatedAt)
	if err != nil {
		return nil, err
	}

	return &newNote, nil
}

func (db *DB) GetNoteByID(ctx context.Context, id int64) (*DBNote, error) {
	query := `
		SELECT id, title, content, created_at, updated_at
		FROM notes
		WHERE id = ?
	`

	row := db.QueryRowContext(ctx, query, id)
	var note DBNote
	err := row.Scan(
		&note.ID,
		&note.Title,
		&note.Content,
		&note.CreatedAt,
		&note.UpdatedAt)
	if err != nil {
		return nil, err
	}

	return &note, nil
}

func (db *DB) GetNotes(ctx context.Context, params GetNoteParams) ([]DBNote, error) {
	query := `
		SELECT id, title, content, created_at, updated_at
		FROM notes
		WHERE title LIKE ?
		LIMIT ?
		OFFSET ?
	`

	rows, err := db.QueryContext(ctx, query, "%"+params.Query+"%", params.Limit, params.Offset)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var notes []DBNote
	for rows.Next() {
		var note DBNote
		if err := rows.Scan(
			&note.ID,
			&note.Title,
			&note.Content,
			&note.CreatedAt,
			&note.UpdatedAt); err != nil {
			return nil, err
		}
		notes = append(notes, note)
	}

	return notes, nil
}

func (db *DB) UpdateNote(ctx context.Context, params UpdateNoteParams) (*DBNote, error) {
	query := `
		UPDATE notes
		SET title = ?, content = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ?
		RETURNING id, title, content, created_at, updated_at
	`

	row := db.QueryRowContext(ctx, query, params.Title, params.Content, params.ID)
	var note DBNote
	err := row.Scan(
		&note.ID,
		&note.Title,
		&note.Content,
		&note.CreatedAt,
		&note.UpdatedAt)
	if err != nil {
		return nil, err
	}

	return &note, nil
}

func (db *DB) DeleteNote(ctx context.Context, id int64) (*DBNote, error) {
	query := `
		DELETE FROM notes
		WHERE id = ?
		RETURNING id, title, content, created_at, updated_at
	`

	row := db.QueryRowContext(ctx, query, id)
	var note DBNote
	err := row.Scan(
		&note.ID,
		&note.Title,
		&note.Content,
		&note.CreatedAt,
		&note.UpdatedAt)
	if err != nil {
		return nil, err
	}

	return &note, nil
}
