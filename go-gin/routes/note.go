package routes

import (
	"go-gin/model"
	"go-gin/routes/dtos"
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"github.com/google/uuid"
	cmap "github.com/orcaman/concurrent-map/v2"
)

var users = model.UserNoteData{
	Users: cmap.New[model.NoteData](),
}

func CreateNote(context *gin.Context) {
	userid := GetUserIdHeader(context)
	if len(userid) == 0 {
		context.String(http.StatusUnauthorized, "")
		return
	}

	noteDate, exist := users.Users.Get(userid)
	if !exist {
		noteDate = model.NoteData{
			Notes: cmap.New[model.Note](),
		}
		users.Users.Set(userid, noteDate)

	}

	var dto dtos.NewNoteDto
	if err := context.ShouldBindBodyWith(&dto, binding.JSON); err != nil {
		log.Printf("%+v", err)
		context.String(http.StatusBadRequest, "Invalid properties")
		return
	}

	id := uuid.New().String()

	noteDate.Notes.Set(id, model.Note{
		Title:     dto.Title,
		Content:   dto.Content,
		CreatedAt: time.Now(),
	})

	context.JSON(http.StatusCreated, gin.H{
		"id": id,
	})
}

func GetAll(context *gin.Context) {
	userid := GetUserIdHeader(context)
	if len(userid) == 0 {
		context.String(http.StatusUnauthorized, "")
		return
	}

	var (
		noteData model.NoteData
		exist    bool
	)

	noteData, exist = users.Users.Get(userid)
	if !exist {
		noteData = model.NoteData{
			Notes: cmap.New[model.Note](),
		}
	}

	context.JSON(http.StatusOK, gin.H{
		"items": noteData.Notes,
	})
}

func GetById(context *gin.Context) {
	userid := GetUserIdHeader(context)
	if len(userid) == 0 {
		context.String(http.StatusUnauthorized, "")
		return
	}

	noteData, exist := users.Users.Get(userid)
	if !exist {
		context.String(http.StatusNotFound, "")
		return
	}

	id := context.Param("id")
	note, exist := noteData.Notes.Get(id)
	if !exist {
		context.String(http.StatusNotFound, "Not found")
	}

	context.JSON(http.StatusOK, gin.H{
		"item": note,
	})
}
