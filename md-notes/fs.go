package main

import (
	"context"
	"encoding/json"
	"os"
	"strings"

	"github.com/gofiber/fiber/v3/log"
)

type FSError string

const (
	ErrNoteAlreadyExists FSError = "note already exists"
	ErrNoteNotFound      FSError = "note not found"
	ErrNoteDeleted       FSError = "note has been deleted"
	ErrNoteNotCreated    FSError = "note has not been created"
)

func (e FSError) Error() string {
	return string(e)
}

type PathBuilder struct {
	parts []string
}

func NewPathBuilder(base string) *PathBuilder {
	parts := strings.Split(base, "/")
	var res []string
	for _, part := range parts {
		if part != "" {
			res = append(res, part)
		}
	}

	return &PathBuilder{parts: res}
}

func (p *PathBuilder) Add(part string) *PathBuilder {
	if part == "" {
		return p
	}
	newParts := make([]string, len(p.parts))
	copy(newParts, p.parts)
	parts := strings.Split(part, "/")
	for _, part := range parts {
		if part != "" {
			newParts = append(newParts, part)
		}
	}
	return &PathBuilder{parts: newParts}
}

func (p *PathBuilder) Concat(other *PathBuilder) *PathBuilder {
	return &PathBuilder{parts: append(p.parts, other.parts...)}
}

func (p *PathBuilder) ConcatFront(other *PathBuilder) *PathBuilder {
	return &PathBuilder{parts: append(other.parts, p.parts...)}
}

func (p *PathBuilder) GetParent() *PathBuilder {
	if len(p.parts) == 0 {
		return &PathBuilder{parts: []string{}}
	}
	return &PathBuilder{parts: p.parts[:len(p.parts)-1]}
}

func (p *PathBuilder) GetName() string {
	if len(p.parts) == 0 {
		return ""
	}
	return p.parts[len(p.parts)-1]
}

func (p *PathBuilder) String() string {
	return strings.Join(p.parts, "/")
}

type FSMeta struct {
	path  string
	len   int
	paths map[int]string
}

func NewFSMeta(pathBuilder *PathBuilder) (*FSMeta, error) {
	metaPath := pathBuilder.Add(".meta.json")

	type rawMeta struct {
		Len   int            `json:"len"`
		Paths map[int]string `json:"paths"`
	}

	bytes, err := json.Marshal(rawMeta{Len: 0, Paths: make(map[int]string)})
	if _, err := os.Stat(metaPath.String()); os.IsNotExist(err) {
		if err := os.WriteFile(metaPath.String(), bytes, 0644); err != nil {
			return nil, err
		}
	}

	bytes, err = os.ReadFile(metaPath.String())
	if err != nil {
		return nil, err
	}

	var meta rawMeta
	if err := json.Unmarshal(bytes, &meta); err != nil {
		return nil, err
	}

	return &FSMeta{path: metaPath.String(), len: meta.Len, paths: meta.Paths}, nil
}

func (m *FSMeta) save() error {
	type rawMeta struct {
		Len   int            `json:"len"`
		Paths map[int]string `json:"paths"`
	}

	raw := rawMeta{Len: m.len, Paths: m.paths}

	bytes, err := json.Marshal(raw)
	if err != nil {
		return err
	}
	if err := os.WriteFile(m.path, bytes, 0644); err != nil {
		return err
	}
	log.Info("Meta updated")
	return nil
}

func (m *FSMeta) Incr(path string) error {
	m.len++
	m.paths[m.len] = path
	log.Infof("New note ID: %v", m.len)
	return m.save()
}

func (m *FSMeta) Decr() error {
	if m.len == 0 {
		return nil
	}
	m.len--
	log.Infof("Deleted note ID: %v", m.len)
	return m.save()
}

type FS struct {
	root *PathBuilder
	meta *FSMeta
}

func NewFS(root string) (*FS, error) {
	rootPath := NewPathBuilder(root)
	if err := os.MkdirAll(rootPath.String(), 0755); err != nil {
		return nil, err
	}

	meta, err := NewFSMeta(rootPath)
	if err != nil {
		return nil, err
	}

	return &FS{root: rootPath, meta: meta}, nil
}

func (f *FS) CreateNote(ctx context.Context, params CreateNoteParams) (*DBNote, error) {
	dirPath := f.root.Concat(params.RelPath)

	if err := os.MkdirAll(dirPath.String(), 0755); err != nil {
		return nil, err
	}

	filePath := dirPath.Add(params.Title + ".md")
	if _, err := os.Stat(filePath.String()); err == nil {
		return nil, ErrNoteAlreadyExists
	}
	err := os.WriteFile(filePath.String(), []byte(params.Content), 0644)
	if err != nil {
		return nil, err
	}

	if err := f.meta.Incr(filePath.String()); err != nil {
		return nil, err
	}

	stat, err := os.Stat(filePath.String())
	if err != nil {
		return nil, err
	}
	return &DBNote{
		ID:        int64(f.meta.len),
		Title:     params.Title,
		Content:   params.Content,
		CreatedAt: stat.ModTime(),
		UpdatedAt: stat.ModTime(),
	}, nil
}
func (f *FS) GetNoteByID(ctx context.Context, id int64) (*DBNote, error) {
	path, ok := f.meta.paths[int(id)]
	if !ok {
		return nil, ErrNoteNotFound
	}
	filePath := NewPathBuilder(path)
	stat, err := os.Stat(path)
	if err != nil {
		return nil, err
	}
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	return &DBNote{
		ID:        id,
		Title:     filePath.GetName(),
		Content:   string(bytes),
		CreatedAt: stat.ModTime(),
		UpdatedAt: stat.ModTime(),
	}, nil
}
func (f *FS) UpdateNote(ctx context.Context, params UpdateNoteParams) (*DBNote, error) {
	panic("not implemented") // TODO: Implement
}
func (f *FS) DeleteNote(ctx context.Context, id int64) (*DBNote, error) {
	panic("not implemented") // TODO: Implement
}
func (f *FS) GetNotes(ctx context.Context, params GetNoteParams) ([]DBNote, error) {
	panic("not implemented") // TODO: Implement
}
