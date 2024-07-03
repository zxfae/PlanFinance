package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	_ "github.com/mattn/go-sqlite3"
)

func InitDB() (*sql.DB, error) {
	dbFilePath := "./entrepreunariat.db"

	db, err := sql.Open("sqlite3", dbFilePath)
	if err != nil {
		return nil, err
	}

	// Configure database connection settings
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

func getUsers(w http.ResponseWriter, r *http.Request) {
	rows, err := Db.Query("SELECT * FROM users")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var users []string
	for rows.Next() {
		var username string
		err = rows.Scan(&username)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		users = append(users, username)
	}

	json.NewEncoder(w).Encode(users)
}

func main() {
	InitMainDb()
	http.HandleFunc("/users", getUsers)
	log.Fatal(http.ListenAndServe(":8080", nil))
}
