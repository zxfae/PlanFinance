package db

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	_ "github.com/mattn/go-sqlite3"
)

var Db *sql.DB

type User struct {
	ID        int    `json:"id"`
	Lastname  string `json:"lastname"`
	Firstname string `json:"firstname"`
}

func InitDB() (*sql.DB, error) {
	dbFilePath := "./db/entrepreunariat.db"

	db, err := sql.Open("sqlite3", dbFilePath)
	if err != nil {
		return nil, err
	}

	db.SetMaxOpenConns(1)
	db.SetMaxIdleConns(1)
	db.SetConnMaxLifetime(0)

	if err := db.Ping(); err != nil {
		db.Close()
		return nil, err
	}

	return db, nil
}

func InitMainDb() {
	var err error

	Db, err = InitDB()
	if err != nil {
		log.Fatalf("Failed to initialize the database %v", err)
	}

	tableCreations := []struct {
		name string
		fn   func(*sql.DB) error
	}{
		{"Users", CreateTableUsers},
	}

	for _, tablefunc := range tableCreations {
		if err := tablefunc.fn(Db); err != nil {
			fmt.Printf("Error Creating table %s: %s\n", tablefunc.name, err)
		} else {
			fmt.Printf("Table created %s successfully\n", tablefunc.name)
		}
	}
	log.Println("Database initialized, test user and test post inserted successfully")
}
func GetUsers(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	if id == "" {
		http.Error(w, "Missing user ID", http.StatusBadRequest)
		return
	}

	var user User
	err := Db.QueryRow("SELECT * FROM Users WHERE id = ?", id).Scan(&user.ID, &user.Lastname, &user.Firstname)
	if err != nil {
		if err == sql.ErrNoRows {
			http.Error(w, "User not found", http.StatusNotFound)
		} else {
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(user)
}
