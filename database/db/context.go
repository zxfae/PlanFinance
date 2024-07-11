package db

import (
	"encoding/json"
	"log"
	"net/http"
)

func AddUsers(w http.ResponseWriter, r *http.Request) {
	var user User
	err := json.NewDecoder(r.Body).Decode(&user)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

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
	log.Printf("User inserted with ID: %d\n", user.ID)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(user)
}
func AddEntreprise(w http.ResponseWriter, r *http.Request) {
	var ent Entreprises
	err := json.NewDecoder(r.Body).Decode(&ent)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	stmt, err := Db.Prepare("INSERT INTO Entreprises(user_id, name, date, codeape, status, jrsttx, jrsweek, jrsferies, jrscp, jan, fev, mar, avr, mai, juin, jui, aout, sept, oct, nov, dec) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()

	result, err := stmt.Exec(ent.UserID, ent.Name, ent.Date, ent.Codeape, ent.Status, ent.Jrsttx, ent.Jrsweek, ent.Jrsferies, ent.Jrscp, ent.Jan, ent.Fev, ent.Mar, ent.Avr, ent.Mai, ent.Juin, ent.Jui, ent.Aout, ent.Sept, ent.Oct, ent.Nov, ent.Dec)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	id, err := result.LastInsertId()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	//Log for get ID and compare with ID user
	ent.ID = int(id)
	log.Printf("Entreprise inserted with ID: %d\n", ent.ID)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(ent)
}

func AddActivites(w http.ResponseWriter, r *http.Request) {
	var act Activites
	err := json.NewDecoder(r.Body).Decode(&act)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	stmt, err := Db.Prepare("INSERT INTO Activites(user_id, production, entretien, clientele, interprofession, formation) VALUES(?, ?, ?, ?, ?, ?)")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()

	result, err := stmt.Exec(act.UserId, act.Production, act.Entretien, act.Clientele, act.Interprofession, act.Formation)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	id, err := result.LastInsertId()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	//Log for get ID and compare with ID user
	act.ID = int(id)
	log.Printf("Act inserted with ID: %d\n", act.ID)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(act)
}
