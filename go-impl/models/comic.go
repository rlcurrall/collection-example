package models

import (
	"encoding/json"
	"net/url"
	"strings"
	"time"

	"gorm.io/gorm"
)

type NewComic struct {
	Title         string   `json:"title" example:"Batman #52 (2011)"`
	IssueNumber   string   `json:"issueNumber" example:"52"`
	MainCharacter string   `json:"mainCharacter" example:"Batman"`
	Genre         string   `json:"genre" example:"superhero"`
	CoverYear     JsonDate `json:"coverYear" example:"2011-02-03"`
	Publisher     string   `json:"publisher" example:"DC"`
	Grade         float32  `json:"grade" example:"9.2"`
	Price         float32  `json:"price" example:"32.21"`
	ImageUrl      string   `json:"imageUrl" example:"https://www.dccomics.com/sites/default/files/styles/covers192x291/public/comic-covers/2018/06/batman_v2_52_5b2ad8fd533996.66775403.jpg"`
}

type Comic struct {
	ID uint `gorm:"primarykey" json:"id"`
	NewComic
	Username  string    `json:"username" example:"cmx_collector42"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

func (c *Comic) BeforeCreate(tx *gorm.DB) (err error) {
	imgUrl, err := url.Parse(c.ImageUrl)
	if err != nil {
		return
	}

	// For demo purposes, could potentially upload image to a central bucket and
	// update the URL to point to that image.
	values := imgUrl.Query()
	values.Add("test", "done")
	imgUrl.RawQuery = values.Encode()

	c.ImageUrl = imgUrl.String()

	return
}

// We need a way to Unmarshal dates in the format of `YYYY-MM-DD`
type JsonDate time.Time

// Implement Marshaler and Unmarshaler interface
func (j *JsonDate) UnmarshalJSON(b []byte) error {
	s := strings.Trim(string(b), "\"")
	t, err := time.Parse("2006-01-02", s)
	if err != nil {
		return err
	}
	*j = JsonDate(t)
	return nil
}

func (j JsonDate) MarshalJSON() ([]byte, error) {
	return json.Marshal(time.Time(j))
}

// Maybe a Format function for printing your date
func (j JsonDate) Format(s string) string {
	t := time.Time(j)
	return t.Format(s)
}
