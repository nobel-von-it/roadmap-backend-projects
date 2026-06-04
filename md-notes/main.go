package main

import (
	"context"
	"log"
)

func main() {
	// db, err := NewDB()
	// if err != nil {
	// 	log.Fatal("failed to connect to database: ", err)
	// }
	//
	// if err := db.InitTables(); err != nil {
	// 	log.Fatal(err)
	// }
	fs, err := NewFS("dev/storage")
	if err != nil {
		log.Fatal(err)
	}

	log.Println(fs.root.String())

	note, err := fs.CreateNote(context.Background(), CreateNoteParams{
		RelPath: NewPathBuilder("test/test2////////////"),
		Note:    Note{Title: "Test", Content: "Test"}})
	if err != nil {
		if err == ErrNoteAlreadyExists {
			log.Println("Note already exists")
		} else {
			log.Fatal(err)
		}
	}

	log.Println(note)

	note, err = fs.GetNoteByID(context.Background(), 1)
	if err != nil {
		if err == ErrNoteNotFound {
			log.Println("Note not found")
		} else {
			log.Fatal(err)
		}
	}
	log.Println(note)
}
