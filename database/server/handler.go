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
	router.HandleFunc("/add_ent", db.AddEntreprise)

	corsRouter := AddCorsHeaders(router)
	serverConfig := ServerParameters(corsRouter)

	log.Fatal(serverConfig.ListenAndServe())
}
