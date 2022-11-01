package database

import (
	"os"
	"time"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var (
	DBConn *gorm.DB
)

func Connect() (err error) {
	databaseUrl := os.Getenv("DATABASE_URL")

	DBConn, err = gorm.Open(postgres.Open(databaseUrl), &gorm.Config{})
	if err != nil {
		return err
	}

	sqlDB, _ := DBConn.DB()
	sqlDB.SetMaxIdleConns(10)
	sqlDB.SetMaxOpenConns(50)
	sqlDB.SetConnMaxLifetime(time.Hour)

	return nil
}
