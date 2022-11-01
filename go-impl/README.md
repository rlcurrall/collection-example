# Simple Collection API Implementation in Go

This example API uses Go to create an app for tracking a users collection. It is
lacking some of the data fields and functionality but is mostly functional, and
provides a Swagger UI documentation explorer available at http://localhost:3000/v1

## Usage
Have Go installed on your machine and run the following in your terminal:
```bash
go run main.go
```

This will start the server listening on port 3000.

## Updating Swagger Docs
If you modify any of the Swagger annotations in the code and need to regenerate
the documentation do the following:

1. Download [swaggo](https://github.com/swaggo/swag) by using:

   ```bash
   $ go install github.com/swaggo/swag/cmd/swag@latest
   ```
2. Run the [swaggo](https://github.com/swaggo/swag) command in your Go project root directory which contain `main.go` file. This command will generate required files (`docs` and `docs/doc.go`)

   ```bash
   $ swag init
   ```
3. Start the server normally:

   ```bash
   go run main.go
   ```
