package main

import (
	"fmt"
	"go-gin/routes"

	"github.com/gin-gonic/gin"
)

func main() {
	port := 3000

	//Running in "debug" mode. Switch to "release" mode in production.
	gin.SetMode(gin.ReleaseMode)

	// Defult have logger
	//router := gin.Default()
	router := gin.New()

	router.GET("/health", routes.GetHealth)

	linkGroup := router.Group("/note")
	linkGroup.POST("", routes.CreateNote)
	linkGroup.GET("", routes.GetAll)
	linkGroup.GET("/:id", routes.GetById)

	router.Run(fmt.Sprintf("0.0.0.0:%d", port))
}
