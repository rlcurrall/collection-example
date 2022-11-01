package handlers

import (
	"fmt"
	"net/http"
	"strings"
	"teki/collection/database"
	"teki/collection/models"

	"github.com/gofiber/fiber/v2"
	"github.com/golang-jwt/jwt/v4"
)

type ErrorResponse struct {
	Message string `json:"message" example:"An error occurred"`
}

// GetAllComics is a function to get all comics data from database
//
// @Summary 	Get all comics
// @Description Get all comics
// @Tags 		comics
// @Accept 		json
// @Produce 	json
// @Param 		username 	 query		string  false  "Owner of the comics"
// @Param 		title	 	 query		string  false  "Title of the comic"
// @Param 		order[price] query		string  false  "Order by price"  Enums(asc, desc)
// @Success 	200 		 {array} 	[]models.Comic
// @Failure 	503 		 {object} 	ErrorResponse{}
// @Router 		/v1/comics 	 [get]
func GetAllComics(c *fiber.Ctx) error {
	db := database.DBConn

	var comics []models.Comic
	query := db.Model(&models.Comic{})
	query.Debug()

	if user := c.Query("username"); user != "" {
		query.Where("username = ?", user)
	}

	if title := c.Query("title"); title != "" {
		query.Where("title ILIKE ?", "%"+title+"%")
	}

	if orderPrice := strings.ToLower(c.Query("order[price]")); orderPrice != "" {
		if orderPrice == "asc" {
			query.Order("price ASC")
		} else if orderPrice == "desc" {
			query.Order("price DESC")
		}
	}

	if res := query.Find(&comics); res.Error != nil {
		return c.Status(http.StatusServiceUnavailable).JSON(ErrorResponse{
			Message: res.Error.Error(),
		})
	}

	return c.JSON(comics)
}

// GetComicByID is a function to get a comic by ID
//
// @Summary 	 Get comic by ID
// @Description  Get comic by ID
// @Tags 		 comics
// @Accept 		 json
// @Produce 	 json
// @Param 		 id 			  path 	 	int  true  "Comic ID"
// @Success 	 200 			  {object}  models.Comic
// @Failure 	 404 			  {object}  ErrorResponse{}
// @Failure 	 503 			  {object}  ErrorResponse{}
// @Router 		 /v1/comics/{id}  [get]
func GetComicById(c *fiber.Ctx) error {
	id := c.Params("id")
	db := database.DBConn

	comic := new(models.Comic)
	if err := db.First(&comic, id).Error; err != nil {
		switch err.Error() {
		case "record not found":
			return c.Status(http.StatusNotFound).JSON(ErrorResponse{
				Message: fmt.Sprintf("Comic with ID %v not found", id),
			})
		default:
			return c.Status(http.StatusServiceUnavailable).JSON(ErrorResponse{
				Message: err.Error(),
			})
		}
	}

	return c.JSON(comic)
}

// AddComic adds a new comic to the users collection
//
// @Summary 	Add a new comic
// @Description Add a new comic
// @Security BearerToken
// @Tags 		comics
// @Accept 		json
// @Produce 	json
// @Param 		comic 	  	body 	  models.NewComic  true  "Add a comic"
// @Success 	201 	  	{object}  models.Comic
// @Failure 	422 	  	{object}  ErrorResponse{}
// @Router 		/v1/comics	[post]
func AddComic(c *fiber.Ctx) error {
	db := database.DBConn

	comic := new(models.Comic)
	if err := c.BodyParser(&comic); err != nil {
		return c.Status(http.StatusUnprocessableEntity).JSON(ErrorResponse{
			Message: err.Error(),
		})
	}

	user := c.Locals("user").(*jwt.Token)
	claims := user.Claims.(jwt.MapClaims)
	username := claims["username"].(string)
	comic.Username = username

	db.Create(comic)

	return c.Status(http.StatusCreated).JSON(comic)
}

// DeleteComic function removes a comic by ID
//
// @Summary 	Remove comic by ID
// @Description Remove comic by ID
// @Security BearerToken
// @Tags 		comics
// @Accept 		json
// @Produce 	json
// @Param 		id  path  int  true  "Comic ID"
// @Success 	200
// @Failure 	404 			 {object}  ErrorResponse{}
// @Failure 	503 			 {object}  ErrorResponse{}
// @Router 		/v1/comics/{id}  [delete]
func DeleteComic(c *fiber.Ctx) error {
	id := c.Params("id")
	db := database.DBConn

	user := c.Locals("user").(*jwt.Token)
	claims := user.Claims.(jwt.MapClaims)
	username := claims["username"].(string)

	comic := new(models.Comic)
	if err := db.First(&comic, id).Error; err != nil {
		switch err.Error() {
		case "record not found":
			return c.Status(http.StatusNotFound).JSON(ErrorResponse{
				Message: fmt.Sprintf("Comic with ID %v not found", id),
			})
		default:
			return c.Status(http.StatusServiceUnavailable).JSON(ErrorResponse{
				Message: err.Error(),
			})
		}
	}

	if comic.Username != username {
		return c.Status(http.StatusForbidden).JSON(ErrorResponse{
			Message: "You are not the owner of this item",
		})
	}

	db.Delete(&comic)

	return nil
}

// UpdateComic function updates a comic by ID
//
// @Summary 	Update comic by ID
// @Description Update comic by ID
// @Security BearerToken
// @Tags 		comics
// @Accept 		json
// @Produce 	json
// @Param 		id    			 path 	  int		  	  true "Comic ID"
// @Param 		comic 			 body 	  models.NewComic true "Add a comic"
// @Success 	200 	  	     {object} models.Comic
// @Failure 	404 			 {object} ErrorResponse{}
// @Failure 	503 			 {object} ErrorResponse{}
// @Router 		/v1/comics/{id}  [patch]
func UpdateComic(c *fiber.Ctx) error {
	id := c.Params("id")
	db := database.DBConn

	user := c.Locals("user").(*jwt.Token)
	claims := user.Claims.(jwt.MapClaims)
	username := claims["username"].(string)

	comic := new(models.Comic)
	if err := db.First(&comic, id).Error; err != nil {
		switch err.Error() {
		case "record not found":
			return c.Status(http.StatusNotFound).JSON(ErrorResponse{
				Message: fmt.Sprintf("Comic with ID %v not found", id),
			})
		default:
			return c.Status(http.StatusServiceUnavailable).JSON(ErrorResponse{
				Message: err.Error(),
			})
		}
	}

	updates := new(models.Comic)
	if err := c.BodyParser(&updates); err != nil {
		return c.Status(http.StatusUnprocessableEntity).JSON(ErrorResponse{
			Message: err.Error(),
		})
	}

	if comic.Username != username {
		return c.Status(http.StatusForbidden).JSON(ErrorResponse{
			Message: "You are not the owner of this item",
		})
	}

	db.Model(&comic).Updates(updates)

	return c.JSON(comic)
}
