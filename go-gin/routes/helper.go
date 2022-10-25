package routes

import (
	"go-gin/model"
	"net/http"

	"github.com/gin-gonic/gin"
)

func GetUsersContext(context *gin.Context) (value *model.UserNoteData, exists bool) {
	data, exist := context.Get("users")
	if !exist {
		context.String(http.StatusBadRequest, "Some error")
		return &model.UserNoteData{}, false
	}

	return data.(*model.UserNoteData), true
}

func GetUserIdHeader(context *gin.Context) string {
	return context.Request.Header.Get("userid")
}
