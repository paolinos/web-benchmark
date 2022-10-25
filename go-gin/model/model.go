package model

import (
	"time"

	cmap "github.com/orcaman/concurrent-map/v2"
)

type Note struct {
	Title     string
	Content   string
	CreatedAt time.Time
}

type NoteData struct {
	Notes cmap.ConcurrentMap[Note]
}

type UserNoteData struct {
	Users cmap.ConcurrentMap[NoteData]
}
