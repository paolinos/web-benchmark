package main

import (
	"go-gin/routes"
	"fmt"

	"github.com/gin-gonic/gin"
)

func main() {
	port := 3000

	//Running in "debug" mode. Switch to "release" mode in production.
	gin.SetMode(gin.ReleaseMode)

	router := gin.Default()
	router.POST("", routes.CreateNote)
	router.GET("", routes.GetAll)
	router.GET("/:id", routes.GetById)

	/*
		linkGroup := router.Group("/note")
		linkGroup.POST("", Get)
		linkGroup.GET("", Post)
		linkGroup.GET("/:id", GetById)
	*/

	router.Run(fmt.Sprintf("0.0.0.0:%d", port))
}