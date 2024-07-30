package db

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strconv"

	_ "github.com/mattn/go-sqlite3"
)

var Db *sql.DB

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
		{"Activites", CreateTableActivite},
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
	userIDStr := r.URL.Query().Get("user_id")
	if userIDStr == "" {
		http.Error(w, "Missing user_id", http.StatusBadRequest)
		return
	}

	userID, err := strconv.Atoi(userIDStr)
	if err != nil {
		http.Error(w, "Invalid user_id", http.StatusBadRequest)
		return
	}

	var entreprise Entreprises
	err = Db.QueryRow(
		`SELECT id, user_id, name, date, codeape, status, jrsttx, jrsweek, jrsferies, jrscp, jan, fev, mar, avr, mai, juin, jui, aout, sept, oct, nov, dec 
		 FROM Entreprises 
		 WHERE user_id = ?`, userID,
	).Scan(
		&entreprise.ID, &entreprise.UserID, &entreprise.Name, &entreprise.Date, &entreprise.Codeape, &entreprise.Status,
		&entreprise.Jrsttx, &entreprise.Jrsweek, &entreprise.Jrsferies, &entreprise.Jrscp, &entreprise.Jan, &entreprise.Fev, &entreprise.Mar, &entreprise.Avr, &entreprise.Mai, &entreprise.Juin, &entreprise.Jui, &entreprise.Aout, &entreprise.Sept, &entreprise.Oct, &entreprise.Nov, &entreprise.Dec,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			http.Error(w, "Entreprise not found", http.StatusNotFound)
		} else {
			http.Error(w, "Database error: "+err.Error(), http.StatusInternalServerError)
		}
		return
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(entreprise); err != nil {
		http.Error(w, "Failed to encode response: "+err.Error(), http.StatusInternalServerError)
	}
}

func GetActivite(w http.ResponseWriter, r *http.Request) {
	userIDStr := r.URL.Query().Get("user_id")
	if userIDStr == "" {
		http.Error(w, "Missing user_id", http.StatusBadRequest)
		return
	}

	userID, err := strconv.Atoi(userIDStr)
	if err != nil {
		http.Error(w, "Invalid user_id", http.StatusBadRequest)
		return
	}

	var act Activites
	err = Db.QueryRow(
		`SELECT id, user_id, production, entretien, clientele, interprofession, formation, prodjour, totalservice, tva, moyprix, donttva, totalmoyprix, htjours, ttcann, tvaann, htcanann 
		 FROM Activites 
		 WHERE user_id = ?`, userID,
	).Scan(
		&act.ID, &act.UserId, &act.Production, &act.Entretien, &act.Clientele, &act.Interprofession, &act.Formation, &act.Prodjour, &act.TotalService,
		&act.Tva, &act.Moyprix, &act.Donttva, &act.Totalmoyprix, &act.Htjours, &act.Ttcann, &act.Tvaann, &act.Htcanann,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			http.Error(w, "Activites not found", http.StatusNotFound)
		} else {
			http.Error(w, "Database error: "+err.Error(), http.StatusInternalServerError)
		}
		return
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(act); err != nil {
		http.Error(w, "Failed to encode response: "+err.Error(), http.StatusInternalServerError)
	}
}
