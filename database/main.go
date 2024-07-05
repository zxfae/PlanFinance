package main

import (
	"database/database/db"
	"database/database/server"
)

func main() {
	db.InitMainDb()
	server.LoadServer()
}
