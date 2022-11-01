package main

import (
	"log"
	"teki/collection/database"
	"teki/collection/models"
	"teki/collection/routes"

	_ "teki/collection/docs"

	"github.com/joho/godotenv"
)

// @title Collection API
// @version 1.0
// @description This is an API for tracking collections
// @contact.name Robb Currall
// @contact.email rlcurrall@gmail.com
// @license.name Apache 2.0
// @license.url http://www.apache.org/licenses/LICENSE-2.0.html
// @BasePath /
// @securityDefinitions.apiKey BearerToken
// @in header
// @name Authorization
func main() {
	if err := godotenv.Load(); err != nil {
		log.Fatal("Error loading .env file")
	}

	if err := database.Connect(); err != nil {
		log.Panic("Can't connect to database: ", err.Error())
	}

	database.DBConn.AutoMigrate(&models.Comic{})

	app := routes.New()
	log.Fatal((app.Listen(":3000")))
}
