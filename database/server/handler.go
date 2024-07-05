package server

import (
	"database/database/db"
	"log"
	"net/http"
)

func LoadServer() {
	db.InitMainDb()

	router := http.NewServeMux()
	router.HandleFunc("/users", db.GetUsers)
	router.HandleFunc("/add_user", db.AddUser)

	corsRouter := AddCorsHeaders(router)
	serverConfig := ServerParameters(corsRouter)

	log.Fatal(serverConfig.ListenAndServe())
}
