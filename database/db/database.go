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

type Entreprises struct {
	ID        int    `json:"id"`
	UserID    int    `json:"user_id"`
	Name      string `json:"name"`
	Date      string `json:"date"`
	Codeape   string `json:"codeape"`
	Status    string `json:"status"`
	Jrsttx    string `json:"jrsttx"`
	Jrsweek   string `json:"jrsweek"`
	Jrsferies string `json:"jrsferies"`
	Jrscp     string `json:"jrscp"`
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
		{"Entreprises", CreateTableEntreprise},
	}

	var msgOk, msgError string

	for _, tablefunc := range tableCreations {
		if err := tablefunc.fn(Db); err != nil {
			msgError += fmt.Sprintf("Error Creating table %s: %s\n", tablefunc.name, err)
		} else {
			msgOk += fmt.Sprintf("Table created %s successfully\n", tablefunc.name)
		}
	}

	if msgError != "" {
		fmt.Println(msgError)
	}

	if msgOk != "" {
		fmt.Println(msgOk)
	}
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

func GetEntreprise(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("user_id")
	if id == "" {
		http.Error(w, "Missing entreprise user_ID", http.StatusBadRequest)
		return
	}
	var entreprise Entreprises
	err := Db.QueryRow("SELECT * FROM Entreprises WHERE user_id = ?", id).Scan(&entreprise.ID, &entreprise.Name, &entreprise.Date, &entreprise.Codeape, &entreprise.Status, &entreprise.UserID)
	if err != nil {
		if err == sql.ErrNoRows {
			http.Error(w, "Entreprise not found", http.StatusNotFound)
		} else {
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
		return
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(entreprise)
}
