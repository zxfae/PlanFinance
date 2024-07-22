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
	router.HandleFunc("/get_ent", db.GetEntreprise)
	router.HandleFunc("/add_act", db.AddActivites)
	router.HandleFunc("/get_act", db.GetActivite)

	corsRouter := AddCorsHeaders(router)
	serverConfig := ServerParameters(corsRouter)

	log.Fatal(serverConfig.ListenAndServe())
}
