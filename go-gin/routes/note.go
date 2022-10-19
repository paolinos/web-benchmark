package routes

import (
	"go-gin/routes/dtos"
	"log"
	"net/http"
	"sync"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"github.com/google/uuid"
)

type Note struct {
	Title     string
	Content   string
	CreatedAt time.Time
}


//---------------------------------------
type MapStruc struct {
	sync.RWMutex
	data map[string]Note
}

func (m *MapStruc) Get(key string) (Note, bool) {
	m.RLock()
	value, exist := m.data[key]
	m.RUnlock()
	if exist {
		return value, true
	}
	return Note{}, false
}

func (m *MapStruc) GetAll() map[string]Note {
	m.RLock()
	// TODO: copy or range?
	data := m.data
	m.RUnlock()

	return data
}

func (m *MapStruc) Set(key string, value *Note) {
	m.Lock()
	m.data[key] = *value
	m.Unlock()
}

//---------------------------------------
var notes = MapStruc{
	data: make(map[string]Note),
}

func GetAll(context *gin.Context) {

	context.JSON(http.StatusOK, gin.H{
		"items": notes.GetAll(),
	})
}

func CreateNote(context *gin.Context) {

	var model dtos.NewNoteDto
	if err := context.ShouldBindBodyWith(&model, binding.JSON); err != nil {
		log.Printf("%+v", err)
		context.String(http.StatusBadRequest, "Invalid properties")
		return
	}

	id := uuid.New().String()

	notes.Set(id, &Note{
		Title: model.Title,
		Content: model.Content,
		CreatedAt: time.Now(),
	})

	context.JSON(http.StatusCreated, gin.H{
		"id": id,
	})
}

func GetById(context *gin.Context) {

	id := context.Param("id")
	value, exist := notes.Get(id)
	if !exist {
		context.String(http.StatusNotFound, "Not found")
	}

	context.JSON(http.StatusOK, gin.H{
		"item": value,
	})
}