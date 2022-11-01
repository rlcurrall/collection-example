package routes

import (
	"teki/collection/handlers"
	"teki/collection/middleware"

	swagger "github.com/arsmn/fiber-swagger/v2"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/gofiber/redirect/v2"
)

func New() *fiber.App {
	app := fiber.New()
	app.Use(cors.New())
	app.Use(logger.New())

	app.Use(redirect.New(redirect.Config{
		Rules: map[string]string{
			"/v1": "/v1/",
		},
		StatusCode: 301,
	}))

	v1 := app.Group("/v1")
	v1.Get("/comics", handlers.GetAllComics)
	v1.Get("/comics/:id", handlers.GetComicById)
	v1.Post("/comics", middleware.Protected(), handlers.AddComic)
	v1.Patch("/comics/:id", middleware.Protected(), handlers.UpdateComic)
	v1.Delete("/comics/:id", middleware.Protected(), handlers.DeleteComic)
	v1.Get("*", swagger.HandlerDefault)

	return app
}
