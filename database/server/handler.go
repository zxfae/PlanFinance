package server

import (
	"database/database/db"
	"log"
	"net/http"
)

func LoadServer() {
	db.InitMainDb()

	router := http.NewServeMux()
	router.HandleFunc("/get_user", db.GetUsers)
	router.HandleFunc("/add_user", db.AddUsers)

	corsRouter := AddCorsHeaders(router)
	serverConfig := ServerParameters(corsRouter)

	log.Fatal(serverConfig.ListenAndServe())
}
