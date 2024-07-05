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
	rows, err := Db.Query("SELECT * FROM Users")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var users []User
	for rows.Next() {
		var user User
		err = rows.Scan(&user.ID, &user.Lastname, &user.Firstname)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		users = append(users, user)
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(users)
}
func AddUser(w http.ResponseWriter, r *http.Request) {
	var user User
	err := json.NewDecoder(r.Body).Decode(&user)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	fmt.Printf("Received user: %+v\n", user)

	stmt, err := Db.Prepare("INSERT INTO Users(lastname, firstname) VALUES(?, ?)")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()

	result, err := stmt.Exec(user.Lastname, user.Firstname)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	id, err := result.LastInsertId()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	user.ID = int(id)
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(user)
}
